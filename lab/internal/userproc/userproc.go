// Package userproc runs the lab's non-network work as the user who invoked
// sudo: processes inside a lab namespace, and the files they and the reports
// need.
package userproc

import (
	"context"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"os/user"
	"path/filepath"
	"strconv"
	"syscall"
	"time"
)

// User is the invoking user.
type User struct {
	UID, GID int
	Name     string
	Home     string
}

// Invoking returns the user who ran `sudo ./bin/lab`.
func Invoking() (User, error) {
	if os.Geteuid() != 0 {
		return User{}, errors.New("run the lab with sudo: it needs root for ip, tc and docker")
	}
	uid, err := strconv.Atoi(os.Getenv("SUDO_UID"))
	if err != nil || uid == 0 {
		return User{}, errors.New("run the lab with sudo from your own account, not as root: SUDO_UID is missing or 0")
	}
	gid, err := strconv.Atoi(os.Getenv("SUDO_GID"))
	if err != nil {
		return User{}, errors.New("SUDO_GID is missing")
	}
	u, err := user.LookupId(strconv.Itoa(uid))
	if err != nil {
		return User{}, err
	}
	return User{UID: uid, GID: gid, Name: u.Username, Home: u.HomeDir}, nil
}

// Command runs name as the user, inside the network namespace ns unless ns
// is empty. The process gets only env, gets SIGTERM when ctx ends, and
// SIGKILL if the lab dies.
func (u User) Command(ctx context.Context, ns string, env []string, dir, name string, args ...string) *exec.Cmd {
	full := append([]string{
		"setpriv", "--reuid", strconv.Itoa(u.UID), "--regid", strconv.Itoa(u.GID), "--init-groups",
		"--pdeathsig", "KILL", "--", name,
	}, args...)
	if ns != "" {
		full = append([]string{"ip", "netns", "exec", ns}, full...)
	}
	cmd := exec.CommandContext(ctx, full[0], full[1:]...)
	cmd.Env = env
	cmd.Dir = dir
	cmd.Cancel = func() error { return cmd.Process.Signal(syscall.SIGTERM) }
	cmd.WaitDelay = 10 * time.Second
	return cmd
}

// MkdirAll creates dir and its missing parents, owned by the user.
func (u User) MkdirAll(dir string) error {
	dir, err := filepath.Abs(dir)
	if err != nil {
		return err
	}
	if _, err := os.Stat(dir); err == nil {
		return nil
	}
	if err := u.MkdirAll(filepath.Dir(dir)); err != nil {
		return err
	}
	if err := os.Mkdir(dir, 0o755); err != nil && !errors.Is(err, os.ErrExist) {
		return err
	}
	return os.Chown(dir, u.UID, u.GID)
}

// WriteFile writes a file owned by the user.
func (u User) WriteFile(path string, data []byte) error {
	if err := u.MkdirAll(filepath.Dir(path)); err != nil {
		return err
	}
	if err := os.WriteFile(path, data, 0o644); err != nil {
		return err
	}
	return os.Chown(path, u.UID, u.GID)
}

// Create creates a file owned by the user and opens it for writing.
func (u User) Create(path string) (*os.File, error) {
	if err := u.MkdirAll(filepath.Dir(path)); err != nil {
		return nil, err
	}
	f, err := os.Create(path)
	if err != nil {
		return nil, err
	}
	if err := f.Chown(u.UID, u.GID); err != nil {
		return nil, errors.Join(fmt.Errorf("chown %s: %w", path, err), f.Close())
	}
	return f, nil
}
