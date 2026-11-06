package main

import (
	"encoding/json"
	"fmt"
	"net"
	"regexp"
	"strings"
)

var ipToken = regexp.MustCompile(`[0-9A-Fa-f:.]{3,}`)

// redactIPs replaces public IP addresses in s; private, CGNAT/tailnet,
// link-local and loopback addresses stay.
func redactIPs(s string) string {
	_, cgnat, _ := net.ParseCIDR("100.64.0.0/10")
	return ipToken.ReplaceAllStringFunc(s, func(tok string) string {
		ip := net.ParseIP(tok)
		if ip == nil || !strings.ContainsAny(tok, ".:") {
			return tok
		}
		if ip.IsPrivate() || ip.IsLoopback() || ip.IsLinkLocalUnicast() || ip.IsUnspecified() || cgnat.Contains(ip) {
			return tok
		}
		if ip.To4() != nil {
			return "<public-v4>"
		}
		return "<public-v6>"
	})
}

func str(v any) string {
	switch t := v.(type) {
	case nil:
		return ""
	case string:
		return t
	default:
		b, _ := json.Marshal(t)
		return string(b)
	}
}

func renderComment(res map[string]any) string {
	var b strings.Builder
	env, _ := res["env"].(map[string]any)
	server, _ := res["server"].(map[string]any)
	tp, _ := server["tailscalePath"].(map[string]any)

	label := str(res["label"])
	title := "Probe result"
	if label != "" {
		title += ": " + label
	}
	fmt.Fprintf(&b, "### %s (posted automatically by the probe server)\n\n", title)
	fmt.Fprintf(&b, "- Run: `%s`, received %s\n", str(res["runId"]), str(server["receivedAt"]))
	fmt.Fprintf(&b, "- OS / browser: %s\n", str(env["os"]))
	fmt.Fprintf(&b, "- Mode: **%s**\n", str(env["mode"]))
	fmt.Fprintf(&b, "- Network (owner-chosen tag, else guess): **%s**\n", str(env["network"]))
	fmt.Fprintf(&b, "- Tailscale path seen by the server when the result arrived: **%s** (peer `%s`, DERP home `%s`)\n", str(tp["path"]), str(tp["peer"]), str(tp["derpHome"]))
	fmt.Fprintf(&b, "- Page loaded over: %s; result posted over: %s\n", str(env["pageProto"]), str(server["resultProto"]))
	fmt.Fprintf(&b, "- User agent: `%s`\n\n", str(env["ua"]))

	b.WriteString("| Test | Result | Details |\n|---|---|---|\n")
	tests, _ := res["tests"].([]any)
	for _, t := range tests {
		m, _ := t.(map[string]any)
		details := strings.ReplaceAll(str(m["summary"]), "|", "\\|")
		details = strings.ReplaceAll(details, "\n", "<br>")
		fmt.Fprintf(&b, "| %s | %s | %s |\n", str(m["name"]), str(m["status"]), details)
	}
	raw, _ := json.MarshalIndent(res, "", "  ")
	b.WriteString("\n<details><summary>Raw result (public IPs redacted)</summary>\n\n```json\n")
	b.WriteString(redactIPs(string(raw)))
	b.WriteString("\n```\n</details>\n")
	return redactIPs(b.String())
}
