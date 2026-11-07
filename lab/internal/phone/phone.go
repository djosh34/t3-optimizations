// Package phone runs the lab's phone: Playwright WebKit in a docker container
// with no network of its own. The lab moves one end of the shaped link into
// the container, and drives the browser with `docker exec`, so Playwright's
// control traffic never crosses the shaped link.
package phone

import (
	"bufio"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"os/exec"
	"strconv"
	"strings"

	"github.com/djosh34/t3-optimizations/lab/internal/netlab"
	"github.com/djosh34/t3-optimizations/lab/internal/userproc"
)

// Image is the driver image, built from lab/driver/Dockerfile.
const Image = "t3lab-phone:1.64.1"

// Config is what the driver gets as its argument.
type Config struct {
	Origin      string `json:"origin"`
	PairURL     string `json:"pairUrl"`
	ThreadID    string `json:"threadId"`
	Prompt      string `json:"prompt"`
	ImagePrompt string `json:"imagePrompt"`
	// Image is the base64 PNG the setup turn attaches.
	Image string `json:"image"`
	// Reply is the normalized text of the measured reply.
	Reply string `json:"reply"`
	// Kind is warm or cold.
	Kind string `json:"kind"`
}

// Event is one line the driver prints.
type Event struct {
	T    int64  `json:"t"`
	Ev   string `json:"ev"`
	N    int    `json:"n,omitempty"`
	Text string `json:"text,omitempty"`
	Name string `json:"name,omitempty"`
	Stats
}

// Stats is what the application received and sent over WebSocket, after
// decompression, and how much of the reply text it received more than once.
type Stats struct {
	RxBytes             int64 `json:"rxBytes,omitempty"`
	TxBytes             int64 `json:"txBytes,omitempty"`
	ReplyBytes          int64 `json:"replyBytes,omitempty"`
	DuplicateReplyBytes int64 `json:"duplicateReplyBytes,omitempty"`
}

// EnsureImage builds the driver image if docker does not have it yet.
func EnsureImage(ctx context.Context, driverDir string) error {
	if _, err := netlab.Run(ctx, "docker", "image", "inspect", Image); err == nil {
		return nil
	}
	fmt.Fprintf(os.Stderr, "lab: building %s\n", Image)
	_, err := netlab.Run(ctx, "docker", "build", "-t", Image, driverDir)
	return err
}

// StartContainer starts the phone container without a network and returns
// the pid whose network namespace the lab attaches. Files the browser writes
// to /out land in outDir, owned by the user.
func StartContainer(ctx context.Context, lab *netlab.Lab, u userproc.User, driverDir, outDir string) (int, error) {
	_, err := netlab.Run(ctx, "docker", "run", "-d", "--name", lab.Container,
		"--network", "none", "--init", "--shm-size", "512m",
		"--user", fmt.Sprintf("%d:%d", u.UID, u.GID), "-e", "HOME=/tmp",
		"-v", driverDir+":/lab:ro", "-v", outDir+":/out",
		Image, "sleep", "infinity")
	if err != nil {
		return 0, err
	}
	out, err := netlab.Run(ctx, "docker", "inspect", "-f", "{{.State.Pid}}", lab.Container)
	if err != nil {
		return 0, err
	}
	return strconv.Atoi(strings.TrimSpace(out))
}

// Driver is the running driver script.
type Driver struct {
	cmd    *exec.Cmd
	stdin  io.WriteCloser
	Events <-chan Event
	done   chan error
}

// StartDriver runs the driver in the container. Its stderr goes to logPath.
func StartDriver(ctx context.Context, lab *netlab.Lab, u userproc.User, cfg Config, logPath string) (*Driver, error) {
	arg, err := json.Marshal(cfg)
	if err != nil {
		return nil, err
	}
	log, err := u.Create(logPath)
	if err != nil {
		return nil, err
	}
	cmd := exec.CommandContext(ctx, "docker", "exec", "-i", lab.Container, "node", "/lab/driver.mjs", string(arg))
	cmd.Stderr = log
	stdin, err := cmd.StdinPipe()
	if err != nil {
		return nil, errors.Join(err, log.Close())
	}
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return nil, errors.Join(err, log.Close())
	}
	if err := cmd.Start(); err != nil {
		return nil, errors.Join(err, log.Close())
	}
	events := make(chan Event, 1024)
	d := &Driver{cmd: cmd, stdin: stdin, Events: events, done: make(chan error, 1)}
	go func() {
		lines := bufio.NewScanner(stdout)
		lines.Buffer(make([]byte, 1<<20), 16<<20)
		for lines.Scan() {
			var ev Event
			if err := json.Unmarshal(lines.Bytes(), &ev); err != nil {
				if _, werr := fmt.Fprintf(log, "not an event: %s\n", lines.Text()); werr != nil {
					fmt.Fprintf(os.Stderr, "lab: driver log: %v\n", werr)
				}
				continue
			}
			events <- ev
		}
		close(events)
		d.done <- errors.Join(cmd.Wait(), log.Close())
	}()
	return d, nil
}

// Send writes a command line to the driver.
func (d *Driver) Send(command string) error {
	_, err := io.WriteString(d.stdin, command+"\n")
	return err
}

// Wait waits for the driver to exit; call it after Events is closed.
func (d *Driver) Wait() error {
	return <-d.done
}
