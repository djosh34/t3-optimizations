// Package netlab builds the lab's network: a server namespace and the phone
// container's namespace, joined by a veth pair with tc netem on both ends.
// Everything it creates is named with the prefix t3lab. It runs ip, tc and
// docker as root, and deletes the server's temp dirs as the user who owns
// them.
package netlab

import (
	"bytes"
	"context"
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"syscall"
	"time"

	"github.com/djosh34/t3-optimizations/lab/internal/profile"
)

// Prefix starts the name of everything the lab creates.
const Prefix = "t3lab"

// Addresses inside the lab. Every lab uses the same ones; they live in the
// lab's own namespaces and never reach the host.
const (
	ServerAddr       = "10.231.0.1"
	PhoneAddr        = "10.231.0.2"
	phoneAddrChanged = "10.231.0.3"
	TunnelServerAddr = "10.231.1.1"
	tunnelPhoneAddr  = "10.231.1.2"
	// BridgeAddr and BridgePeerAddr are the two ends of a bridge between
	// two labs' server namespaces (Bridge).
	BridgeAddr     = "10.231.9.1"
	BridgePeerAddr = "10.231.9.2"
)

// Fixed MAC addresses of the veth ends, so each side's neighbor entries can
// be set up front. Without them the kernel resolves addresses with ARP over
// the shaped link; at heavy loss an entry fails now and then and the kernel
// drops what it queued for it, which a real phone link does not do.
const (
	serverMAC = "02:00:0a:e7:00:01"
	phoneMAC  = "02:00:0a:e7:00:02"
)

// ownerDir holds one file per running lab, named by its id, with the lab's pid.
const ownerDir = "/run/t3lab"

// Lab is one set of namespaces, links and the phone container.
type Lab struct {
	ID        string
	ServerNS  string
	PhoneNS   string
	Container string
	serverIf  string
	phoneIf   string
	tunnelIf  string
}

// New names a lab. id is six hex digits.
func New(id string) *Lab {
	return &Lab{
		ID:        id,
		ServerNS:  Prefix + id + "srv",
		PhoneNS:   Prefix + id + "phone",
		Container: Prefix + id + "phone",
		serverIf:  Prefix + id + "s",
		phoneIf:   Prefix + id + "p",
		tunnelIf:  Prefix + id + "t",
	}
}

// Run runs a root command and returns its output.
func Run(ctx context.Context, name string, args ...string) (string, error) {
	var stdout, stderr bytes.Buffer
	cmd := exec.CommandContext(ctx, name, args...)
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	if err := cmd.Run(); err != nil {
		return stdout.String(), fmt.Errorf("%s %s: %w: %s", name, strings.Join(args, " "), err, strings.TrimSpace(stderr.String()))
	}
	return stdout.String(), nil
}

func run(ctx context.Context, name string, args ...string) error {
	_, err := Run(ctx, name, args...)
	return err
}

// Claim records this process as the owner of a new lab and returns it. The
// id is taken exclusively, so two labs never share one, and a cleanup in
// another lab process leaves this one alone.
func Claim() (*Lab, error) {
	if err := os.MkdirAll(ownerDir, 0o755); err != nil {
		return nil, err
	}
	for range 10 {
		id := make([]byte, 3)
		if _, err := rand.Read(id); err != nil {
			return nil, err
		}
		l := New(hex.EncodeToString(id))
		f, err := os.OpenFile(filepath.Join(ownerDir, l.ID), os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o644)
		if errors.Is(err, os.ErrExist) {
			continue
		}
		if err != nil {
			return nil, err
		}
		_, err = f.WriteString(strconv.Itoa(os.Getpid()))
		if err := errors.Join(err, f.Close()); err != nil {
			return nil, err
		}
		return l, nil
	}
	return nil, errors.New("no free lab id after 10 tries")
}

// Reap waits until the lab's owner process ends, then deletes what the lab
// left. The lab starts it as a separate process, so its namespaces and
// container go away even when the lab is killed. A lab that cleaned up
// itself removed its owner file, and then there is nothing to do.
func Reap(ctx context.Context, id string, pid int) error {
	owner := filepath.Join(ownerDir, id)
	proc := filepath.Join("/proc", strconv.Itoa(pid))
	for {
		if _, err := os.Stat(owner); err != nil {
			return nil
		}
		if _, err := os.Stat(proc); err != nil {
			fmt.Fprintf(os.Stderr, "lab: lab %s ended without cleaning up; removing what it left\n", id)
			return deleteID(ctx, id)
		}
		time.Sleep(time.Second)
	}
}

// CreateServer creates the server namespace and the veth pair. Both ends
// start in the server namespace; AttachPhone moves one into the container.
// GSO is off so that netem sees wire-sized packets, not 64 KB batches.
func (l *Lab) CreateServer(ctx context.Context) error {
	ns := l.ServerNS
	steps := [][]string{
		{"ip", "netns", "add", ns},
		{"ip", "-n", ns, "link", "set", "lo", "up"},
		{"ip", "-n", ns, "link", "add", l.serverIf, "address", serverMAC, "gso_max_segs", "1", "gso_max_size", "1500",
			"type", "veth", "peer", "name", l.phoneIf, "address", phoneMAC, "gso_max_segs", "1", "gso_max_size", "1500"},
		{"ip", "-n", ns, "addr", "add", ServerAddr + "/24", "dev", l.serverIf},
		{"ip", "-n", ns, "neigh", "replace", PhoneAddr, "lladdr", phoneMAC, "nud", "permanent", "dev", l.serverIf},
		{"ip", "-n", ns, "neigh", "replace", phoneAddrChanged, "lladdr", phoneMAC, "nud", "permanent", "dev", l.serverIf},
		{"ip", "-n", ns, "link", "set", l.serverIf, "up"},
		{"tc", "-n", ns, "qdisc", "add", "dev", l.serverIf, "clsact"},
	}
	for _, step := range steps {
		if err := run(ctx, step[0], step[1:]...); err != nil {
			return err
		}
	}
	return nil
}

// AttachPhone names the container's network namespace and moves the phone
// end of the veth pair into it.
func (l *Lab) AttachPhone(ctx context.Context, pid int) error {
	ns := l.PhoneNS
	steps := [][]string{
		{"ip", "netns", "attach", ns, strconv.Itoa(pid)},
		{"ip", "-n", l.ServerNS, "link", "set", l.phoneIf, "netns", ns},
		{"ip", "-n", ns, "addr", "add", PhoneAddr + "/24", "dev", l.phoneIf},
		{"ip", "-n", ns, "link", "set", l.phoneIf, "up"},
		{"ip", "-n", ns, "neigh", "replace", ServerAddr, "lladdr", serverMAC, "nud", "permanent", "dev", l.phoneIf},
		{"ip", "-n", ns, "link", "set", "lo", "up"},
		{"tc", "-n", ns, "qdisc", "add", "dev", l.phoneIf, "clsact"},
	}
	for _, step := range steps {
		if err := run(ctx, step[0], step[1:]...); err != nil {
			return err
		}
	}
	return nil
}

// CreateTunnel adds a TUN device in each namespace, owned by uid, for the
// TCP outer path. The tunnel processes run as that user.
func (l *Lab) CreateTunnel(ctx context.Context, uid int) error {
	for _, side := range []struct{ ns, addr string }{
		{l.ServerNS, TunnelServerAddr},
		{l.PhoneNS, tunnelPhoneAddr},
	} {
		steps := [][]string{
			{"ip", "-n", side.ns, "tuntap", "add", "dev", l.tunnelIf, "mode", "tun", "user", strconv.Itoa(uid)},
			{"ip", "-n", side.ns, "addr", "add", side.addr + "/30", "dev", l.tunnelIf},
			// Tailscale's MTU.
			{"ip", "-n", side.ns, "link", "set", l.tunnelIf, "mtu", "1280", "up"},
		}
		for _, step := range steps {
			if err := run(ctx, step[0], step[1:]...); err != nil {
				return err
			}
		}
	}
	return nil
}

// TunnelIf is the name of the TUN device in both namespaces.
func (l *Lab) TunnelIf() string { return l.tunnelIf }

// Shape applies netem to both directions: down on the server end, up on the
// phone end. A FIFO under netem keeps jitter from reordering packets, unless
// the link asks for reordering.
func (l *Lab) Shape(ctx context.Context, down, up profile.Link, seed int64) error {
	for _, side := range []struct {
		ns, dev string
		link    profile.Link
		seed    int64
	}{
		{l.ServerNS, l.serverIf, down, seed},
		{l.PhoneNS, l.phoneIf, up, seed + 1},
	} {
		// A replace keeps an old FIFO child; start from an empty root.
		if _, err := Run(ctx, "tc", "-n", side.ns, "qdisc", "del", "dev", side.dev, "root"); err != nil &&
			!strings.Contains(err.Error(), "No such file") && !strings.Contains(err.Error(), "Cannot delete qdisc with handle of zero") {
			return err
		}
		args := append([]string{"-n", side.ns, "qdisc", "add", "dev", side.dev, "root", "handle", "10:", "netem"}, side.link.NetemArgs(side.seed)...)
		if err := run(ctx, "tc", args...); err != nil {
			return err
		}
		if len(side.link.Reorder) == 0 {
			if err := run(ctx, "tc", "-n", side.ns, "qdisc", "add", "dev", side.dev, "parent", "10:1", "handle", "20:",
				"pfifo", "limit", strconv.Itoa(side.link.Limit)); err != nil {
				return err
			}
		}
	}
	return nil
}

// Reshape changes both directions' netem in place, keeping what is queued,
// as a phone's link changes under it.
func (l *Lab) Reshape(ctx context.Context, down, up profile.Link, seed int64) error {
	for _, side := range []struct {
		ns, dev string
		link    profile.Link
		seed    int64
	}{
		{l.ServerNS, l.serverIf, down, seed},
		{l.PhoneNS, l.phoneIf, up, seed + 1},
	} {
		args := append([]string{"-n", side.ns, "qdisc", "change", "dev", side.dev, "root", "handle", "10:", "netem"}, side.link.NetemArgs(side.seed)...)
		if err := run(ctx, "tc", args...); err != nil {
			return err
		}
		if err := run(ctx, "tc", "-n", side.ns, "qdisc", "change", "dev", side.dev, "parent", "10:1", "handle", "20:",
			"pfifo", "limit", strconv.Itoa(side.link.Limit)); err != nil {
			return err
		}
	}
	return nil
}

// Bridge joins this lab's server namespace to another's with an unshaped
// veth pair: BridgeAddr here, BridgePeerAddr there. Deleting this lab
// deletes the pair.
func (l *Lab) Bridge(ctx context.Context, peer *Lab) error {
	here, there := Prefix+l.ID+"x", Prefix+l.ID+"y"
	steps := [][]string{
		{"ip", "-n", l.ServerNS, "link", "add", here, "type", "veth", "peer", "name", there, "netns", peer.ServerNS},
		{"ip", "-n", l.ServerNS, "addr", "add", BridgeAddr + "/30", "dev", here},
		{"ip", "-n", peer.ServerNS, "addr", "add", BridgePeerAddr + "/30", "dev", there},
		{"ip", "-n", l.ServerNS, "link", "set", here, "up"},
		{"ip", "-n", peer.ServerNS, "link", "set", there, "up"},
	}
	for _, step := range steps {
		if err := run(ctx, step[0], step[1:]...); err != nil {
			return err
		}
	}
	return nil
}

// SetOutage silences the link in both directions, or ends the silence. It
// drops at each end's ingress, after the shaped queues, so packets already
// in flight are lost too.
func (l *Lab) SetOutage(ctx context.Context, down bool) error {
	for _, side := range []struct{ ns, dev string }{{l.ServerNS, l.serverIf}, {l.PhoneNS, l.phoneIf}} {
		var err error
		if down {
			err = run(ctx, "tc", "-n", side.ns, "filter", "add", "dev", side.dev, "ingress", "pref", "1", "matchall", "action", "drop")
		} else {
			err = run(ctx, "tc", "-n", side.ns, "filter", "del", "dev", side.dev, "ingress", "pref", "1")
		}
		if err != nil {
			return err
		}
	}
	return nil
}

// Block drops one kind of traffic between the phone and the server at each
// end's ingress, or lets it through again: "udp" (the data channel) or
// "https" (TCP to the server's port 8443, the lab front).
func (l *Lab) Block(ctx context.Context, what string, on bool) error {
	pref, match := "", []string{}
	switch what {
	case "udp":
		pref, match = "2", []string{"protocol", "ip", "flower", "ip_proto", "udp"}
	case "https":
		pref = "3"
	default:
		return fmt.Errorf("block: unknown traffic %q", what)
	}
	for _, side := range []struct{ ns, dev, port string }{
		{l.ServerNS, l.serverIf, "dst_port"}, {l.PhoneNS, l.phoneIf, "src_port"},
	} {
		m := match
		if what == "https" {
			m = []string{"protocol", "ip", "flower", "ip_proto", "tcp", side.port, "8443"}
		}
		args := []string{"-n", side.ns, "filter", "del", "dev", side.dev, "ingress", "pref", pref}
		if on {
			args = append([]string{"-n", side.ns, "filter", "add", "dev", side.dev, "ingress", "pref", pref}, append(m, "action", "drop")...)
		}
		if err := run(ctx, "tc", args...); err != nil {
			return err
		}
	}
	return nil
}

// ChangePhoneAddr gives the phone a new address and removes the old one, so
// its open connections break the way they do on a new NAT mapping.
func (l *Lab) ChangePhoneAddr(ctx context.Context) error {
	if err := run(ctx, "ip", "-n", l.PhoneNS, "addr", "add", phoneAddrChanged+"/24", "dev", l.phoneIf); err != nil {
		return err
	}
	return run(ctx, "ip", "-n", l.PhoneNS, "addr", "del", PhoneAddr+"/24", "dev", l.phoneIf)
}

// Counters are the bytes on the wire at the server end of the link.
type Counters struct {
	Down, Up uint64
}

// ReadCounters reads the server end's byte counters: what it sent went down
// to the phone, what it received came up from the phone.
func (l *Lab) ReadCounters(ctx context.Context) (Counters, error) {
	out, err := Run(ctx, "ip", "-n", l.ServerNS, "-s", "-j", "link", "show", "dev", l.serverIf)
	if err != nil {
		return Counters{}, err
	}
	var links []struct {
		Stats64 struct {
			RX struct {
				Bytes uint64 `json:"bytes"`
			} `json:"rx"`
			TX struct {
				Bytes uint64 `json:"bytes"`
			} `json:"tx"`
		} `json:"stats64"`
	}
	if err := json.Unmarshal([]byte(out), &links); err != nil {
		return Counters{}, fmt.Errorf("parse link stats: %w", err)
	}
	if len(links) != 1 {
		return Counters{}, fmt.Errorf("expected one link, got %d", len(links))
	}
	return Counters{Down: links[0].Stats64.TX.Bytes, Up: links[0].Stats64.RX.Bytes}, nil
}

// Delete removes the lab's container, namespaces and owner file. Deleting
// the server namespace deletes the veth pair with it. It tries every step
// and returns every error.
func (l *Lab) Delete(ctx context.Context) error {
	return deleteID(ctx, l.ID)
}

func deleteID(ctx context.Context, id string) error {
	l := New(id)
	errs := []error{removeTempDirs(ctx, id)}
	if out, err := Run(ctx, "docker", "ps", "-aq", "--filter", "name=^"+l.Container+"$"); err != nil {
		errs = append(errs, err)
	} else if strings.TrimSpace(out) != "" {
		errs = append(errs, run(ctx, "docker", "rm", "-f", l.Container))
	}
	names, err := netnsNames(ctx)
	errs = append(errs, err)
	for _, ns := range []string{l.PhoneNS, l.ServerNS} {
		for _, name := range names {
			if name == ns {
				errs = append(errs, run(ctx, "ip", "netns", "del", ns))
			}
		}
	}
	if err := os.Remove(filepath.Join(ownerDir, id)); err != nil && !errors.Is(err, os.ErrNotExist) {
		errs = append(errs, err)
	}
	return errors.Join(errs...)
}

// TempDir holds the labs' temp dirs. It is fixed, not $TMPDIR, so the user
// who creates them and root who cleans up look in the same place.
const TempDir = "/tmp"

// TempDirPattern is the mktemp template for a lab's temp dirs in TempDir.
func TempDirPattern(id string) string { return filepath.Join(TempDir, Prefix+id+"-base-XXXXXX") }

// removeTempDirs deletes a lab's temp dirs, each as the user who owns it, so
// root never deletes inside a user's directory.
func removeTempDirs(ctx context.Context, id string) error {
	dirs, err := filepath.Glob(filepath.Join(TempDir, Prefix+id+"-base-*"))
	if err != nil {
		return err
	}
	var errs []error
	for _, dir := range dirs {
		info, err := os.Lstat(dir)
		if err != nil {
			errs = append(errs, err)
			continue
		}
		st, ok := info.Sys().(*syscall.Stat_t)
		if !ok {
			errs = append(errs, fmt.Errorf("no owner for %s", dir))
			continue
		}
		errs = append(errs, run(ctx, "setpriv", "--reuid", strconv.Itoa(int(st.Uid)), "--regid", strconv.Itoa(int(st.Gid)),
			"--clear-groups", "--", "rm", "-rf", "--", dir))
	}
	return errors.Join(errs...)
}

func netnsNames(ctx context.Context) ([]string, error) {
	out, err := Run(ctx, "ip", "netns", "list")
	if err != nil {
		return nil, err
	}
	var names []string
	for _, line := range strings.Split(out, "\n") {
		if fields := strings.Fields(line); len(fields) > 0 {
			names = append(names, fields[0])
		}
	}
	return names, nil
}

var idPattern = regexp.MustCompile("^" + Prefix + "([0-9a-f]{6})")

// Cleanup removes what earlier labs left behind: everything with the t3lab
// prefix whose owning process is gone. Labs still running are left alone.
func Cleanup(ctx context.Context) error {
	ids := map[string]bool{}
	names, err := netnsNames(ctx)
	if err != nil {
		return err
	}
	containers, err := Run(ctx, "docker", "ps", "-a", "--filter", "name="+Prefix, "--format", "{{.Names}}")
	if err != nil {
		return err
	}
	names = append(names, strings.Fields(containers)...)
	tempDirs, err := filepath.Glob(filepath.Join(TempDir, Prefix+"*-base-*"))
	if err != nil {
		return err
	}
	for _, dir := range tempDirs {
		names = append(names, filepath.Base(dir))
	}
	if entries, err := os.ReadDir(ownerDir); err == nil {
		for _, e := range entries {
			names = append(names, Prefix+e.Name())
		}
	}
	for _, name := range names {
		if m := idPattern.FindStringSubmatch(name); m != nil {
			ids[m[1]] = true
		}
	}
	var errs []error
	for id := range ids {
		if ownerAlive(id) {
			continue
		}
		fmt.Fprintf(os.Stderr, "lab: removing leftovers of lab %s\n", id)
		errs = append(errs, deleteID(ctx, id))
	}
	return errors.Join(errs...)
}

func ownerAlive(id string) bool {
	data, err := os.ReadFile(filepath.Join(ownerDir, id))
	if err != nil {
		return false
	}
	pid, err := strconv.Atoi(strings.TrimSpace(string(data)))
	if err != nil || pid <= 0 {
		return false
	}
	if pid == os.Getpid() {
		return true
	}
	// /proc/<pid> exists while the process runs; signal 0 would need the
	// same user, and the owner is root like us anyway.
	_, err = os.Stat(filepath.Join("/proc", strconv.Itoa(pid)))
	return err == nil
}
