// Package userproc runs the lab's non-network work as the user who invoked
// sudo: processes inside a lab namespace, and the files they and the reports
// need.
package userproc

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"io"
	"os"
	"os/exec"
	"os/user"
	"path/filepath"
	"strconv"
	"strings"
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

// MkdirAll creates dir and its missing parents as the user.
func (u User) MkdirAll(dir string) error {
	var stderr bytes.Buffer
	cmd := u.Command(context.Background(), "", userEnv, "/", "mkdir", "-p", "--", dir)
	cmd.Stderr = &stderr
	if err := cmd.Run(); err != nil {
		return fmt.Errorf("mkdir %s: %w: %s", dir, err, strings.TrimSpace(stderr.String()))
	}
	return nil
}

var userEnv = []string{"PATH=/usr/bin:/bin"}

// Create opens path for writing. The lab runs as root, so a process running
// as the user writes the file: root never opens files in the user's
// directories.
func (u User) Create(path string) (io.WriteCloser, error) {
	if err := u.MkdirAll(filepath.Dir(path)); err != nil {
		return nil, err
	}
	f := &userFile{path: path}
	f.cmd = u.Command(context.Background(), "", userEnv, "/", "/bin/sh", "-c", `exec cat > "$1"`, "sh", path)
	f.cmd.Stderr = &f.stderr
	stdin, err := f.cmd.StdinPipe()
	if err != nil {
		return nil, err
	}
	f.stdin = stdin
	if err := f.cmd.Start(); err != nil {
		return nil, fmt.Errorf("open %s: %w", path, err)
	}
	return f, nil
}

type userFile struct {
	path   string
	cmd    *exec.Cmd
	stdin  io.WriteCloser
	stderr bytes.Buffer
}

func (f *userFile) Write(p []byte) (int, error) { return f.stdin.Write(p) }

// Close ends the file and reports any error the writer had.
func (f *userFile) Close() error {
	err := f.stdin.Close()
	if werr := f.cmd.Wait(); werr != nil {
		err = errors.Join(err, fmt.Errorf("write %s: %w: %s", f.path, werr, strings.TrimSpace(f.stderr.String())))
	}
	return err
}

// WriteFile writes a file as the user.
func (u User) WriteFile(path string, data []byte) error {
	f, err := u.Create(path)
	if err != nil {
		return err
	}
	_, err = f.Write(data)
	return errors.Join(err, f.Close())
}

// Install copies an executable as the user.
func (u User) Install(src, dst string) error {
	var stderr bytes.Buffer
	cmd := u.Command(context.Background(), "", userEnv, "/", "install", "-m", "755", "--", src, dst)
	cmd.Stderr = &stderr
	if err := cmd.Run(); err != nil {
		return fmt.Errorf("install %s: %w: %s", dst, err, strings.TrimSpace(stderr.String()))
	}
	return nil
}
