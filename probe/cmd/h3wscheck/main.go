// h3wscheck opens a WebSocket over HTTP/3 (RFC 9220) to the probe server and
// echoes a few messages. No browser on the probe host speaks RFC 9220, so this
// checks the server side of test 4.
package main

import (
	"context"
	"crypto/tls"
	"fmt"
	"log"
	"net"
	"net/http"
	"net/url"
	"os"
	"time"

	"github.com/gobwas/ws"
	"github.com/gobwas/ws/wsutil"
	"github.com/quic-go/quic-go"
	"github.com/quic-go/quic-go/http3"
)

func main() {
	target := "https://lima-opencode-vm.tail2fc05a.ts.net:6443/ws?id=h3wscheck"
	if len(os.Args) > 1 {
		target = os.Args[1]
	}
	u, _ := url.Parse(target)
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	udp, err := net.ListenUDP("udp", nil)
	if err != nil {
		log.Fatal(err)
	}
	raddr, err := net.ResolveUDPAddr("udp", u.Host)
	if err != nil {
		log.Fatal(err)
	}
	qc, err := quic.Dial(ctx, udp, raddr, &tls.Config{ServerName: u.Hostname(), NextProtos: []string{http3.NextProtoH3}}, &quic.Config{})
	if err != nil {
		log.Fatal(err)
	}
	cc := (&http3.Transport{}).NewClientConn(qc)
	select {
	case <-cc.ReceivedSettings():
	case <-ctx.Done():
		log.Fatal("no settings")
	}
	fmt.Println("server SETTINGS extended CONNECT:", cc.Settings().EnableExtendedConnect)
	str, err := cc.OpenRequestStream(ctx)
	if err != nil {
		log.Fatal(err)
	}
	req := (&http.Request{Method: http.MethodConnect, Proto: "websocket", Host: u.Host, URL: u, Header: http.Header{"Sec-Websocket-Version": {"13"}}}).WithContext(ctx)
	if err := str.SendRequestHeader(req); err != nil {
		log.Fatal(err)
	}
	rsp, err := str.ReadResponse()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("CONNECT status:", rsp.StatusCode)
	for i := 0; i < 5; i++ {
		t := time.Now()
		if err := wsutil.WriteClientMessage(str, ws.OpText, fmt.Appendf(nil, "hello %d", i)); err != nil {
			log.Fatal(err)
		}
		msg, _, err := wsutil.ReadServerData(str)
		if err != nil {
			log.Fatal(err)
		}
		fmt.Printf("echo %q in %v\n", msg, time.Since(t).Round(time.Microsecond))
	}
}
