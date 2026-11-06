// Package tunnel carries IP packets between two TUN devices inside one TCP
// connection, like a Tailscale DERP relay carries WireGuard packets. Loss on
// the link under it then stalls everything in the tunnel, as on DERP.
package tunnel

import (
	"context"
	"encoding/binary"
	"errors"
	"fmt"
	"io"
	"net"
	"os"
	"sync"
	"syscall"
	"time"
	"unsafe"
)

const (
	tunSetIff = 0x400454ca
	iffTUN    = 0x0001
	iffNoPI   = 0x1000
)

// openTUN attaches to an existing persistent TUN device. The lab creates it
// with `ip tuntap add ... user <uid>`, so the user may attach without root.
func openTUN(name string) (*os.File, error) {
	f, err := os.OpenFile("/dev/net/tun", os.O_RDWR, 0)
	if err != nil {
		return nil, err
	}
	var ifr [40]byte
	copy(ifr[:16], name)
	binary.NativeEndian.PutUint16(ifr[16:], iffTUN|iffNoPI)
	if _, _, errno := syscall.Syscall(syscall.SYS_IOCTL, f.Fd(), tunSetIff, uintptr(unsafe.Pointer(&ifr[0]))); errno != 0 {
		return nil, errors.Join(fmt.Errorf("attach to %s: %w", name, errno), f.Close())
	}
	return f, nil
}

// closeLogged closes c and reports any error except an earlier close.
func closeLogged(c io.Closer) {
	if err := c.Close(); err != nil && !errors.Is(err, net.ErrClosed) && !errors.Is(err, os.ErrClosed) {
		fmt.Fprintf(os.Stderr, "tunnel: close: %v\n", err)
	}
}

type tunnel struct {
	tun     *os.File
	mu      sync.Mutex
	current net.Conn
}

// use makes conn the connection that carries packets, closing the old one.
func (t *tunnel) use(conn net.Conn) {
	t.mu.Lock()
	defer t.mu.Unlock()
	if t.current != nil {
		closeLogged(t.current)
	}
	t.current = conn
}

// Run moves packets until ctx ends. With listen set it accepts the TCP
// connection (server side); otherwise it dials connect and redials whenever
// the connection breaks (phone side).
func Run(ctx context.Context, dev, listen, connect string) error {
	tun, err := openTUN(dev)
	if err != nil {
		return err
	}
	t := &tunnel{tun: tun}
	defer func() {
		t.use(nil)
		closeLogged(tun)
	}()
	go t.fromTUN()

	if listen != "" {
		ln, err := net.Listen("tcp", listen)
		if err != nil {
			return err
		}
		go func() {
			<-ctx.Done()
			closeLogged(ln)
		}()
		for {
			conn, err := ln.Accept()
			if err != nil {
				if ctx.Err() != nil {
					return nil
				}
				return err
			}
			t.use(conn)
			go t.toTUN(conn)
		}
	}
	dialer := net.Dialer{Timeout: 10 * time.Second}
	for ctx.Err() == nil {
		conn, err := dialer.DialContext(ctx, "tcp", connect)
		if err != nil {
			time.Sleep(time.Second)
			continue
		}
		t.use(conn)
		t.toTUN(conn)
	}
	return nil
}

// fromTUN sends each packet from the TUN device as a 2-byte length and the
// packet. A blocked TCP connection blocks the device, as on DERP.
func (t *tunnel) fromTUN() {
	buf := make([]byte, 2+65535)
	for {
		n, err := t.tun.Read(buf[2:])
		if err != nil {
			fmt.Fprintf(os.Stderr, "tunnel: read device: %v\n", err)
			return
		}
		t.mu.Lock()
		conn := t.current
		t.mu.Unlock()
		if conn == nil {
			continue
		}
		binary.BigEndian.PutUint16(buf, uint16(n))
		if _, err := conn.Write(buf[:2+n]); err != nil {
			closeLogged(conn)
		}
	}
}

func (t *tunnel) toTUN(conn net.Conn) {
	buf := make([]byte, 65535)
	var size [2]byte
	for {
		if _, err := io.ReadFull(conn, size[:]); err != nil {
			break
		}
		n := binary.BigEndian.Uint16(size[:])
		if _, err := io.ReadFull(conn, buf[:n]); err != nil {
			break
		}
		if _, err := t.tun.Write(buf[:n]); err != nil {
			fmt.Fprintf(os.Stderr, "tunnel: write device: %v\n", err)
			break
		}
	}
	closeLogged(conn)
}
