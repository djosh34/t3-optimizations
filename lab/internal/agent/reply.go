package agent

import (
	"bytes"
	"fmt"
	"image"
	"image/color"
	"image/png"
	"strings"
)

// A Step is one part of a scripted reply: streamed text, an approval request,
// or an image.
type Step struct {
	Text     string
	Approval string
	Image    []byte
}

// Prompts that the lab sends to set up a thread before a measured run.
const (
	WarmupPrompt = "lab:warmup"
	ImagePrompt  = "lab:image"
)

// The prompt the phone sends in every measured run, 64 bytes long.
const MeasuredPrompt = "Please explain how the sync loop recovers after a long outage..."

// Script returns the reply to a prompt. soakParts repeats the middle of the
// standard reply that many extra times for the long soak turn.
func Script(prompt string, soakParts int) []Step {
	switch {
	case strings.Contains(prompt, WarmupPrompt):
		return []Step{{Text: "Ready. The lab thread is set up."}}
	case strings.Contains(prompt, ImagePrompt):
		return []Step{{Text: "I received the image."}}
	}
	steps := []Step{
		{Text: paragraphOne},
		{Text: paragraphTwo},
		{Approval: "Write notes/recovery.md"},
		{Image: Picture()},
		{Text: paragraphThree},
	}
	for part := 1; part <= soakParts; part++ {
		steps = append(steps, Step{Text: fmt.Sprintf("Part %d. %s", part, soakParagraph)})
	}
	return append(steps, Step{Text: unclosedFence})
}

// ReplyText is the text of a reply in the order the agent streams it.
func ReplyText(steps []Step) string {
	var b strings.Builder
	for _, step := range steps {
		b.WriteString(step.Text)
	}
	return b.String()
}

// Normalize keeps only ASCII letters and digits. The lab compares text the
// agent produced with text the phone shows; rendering changes whitespace and
// markdown punctuation but keeps letters and digits.
func Normalize(text string) string {
	var b strings.Builder
	for _, r := range text {
		if r >= 'a' && r <= 'z' || r >= 'A' && r <= 'Z' || r >= '0' && r <= '9' {
			b.WriteRune(r)
		}
	}
	return b.String()
}

// Picture is a deterministic 48x48 PNG of noise, about 7 KB, the image in every
// measured reply. Noise keeps it from compressing to nothing.
func Picture() []byte {
	const size = 48
	img := image.NewRGBA(image.Rect(0, 0, size, size))
	seed := uint32(1)
	for y := range size {
		for x := range size {
			seed = seed*1664525 + 1013904223
			img.Set(x, y, color.RGBA{R: uint8(seed >> 24), G: uint8(seed >> 16), B: uint8(x * 5), A: 255})
		}
	}
	var buf bytes.Buffer
	// Encoding to memory only fails on an invalid image, which this is not.
	if err := png.Encode(&buf, img); err != nil {
		return nil
	}
	return buf.Bytes()
}

// The paragraphs avoid ordered lists, links and tables: their rendering adds or
// drops letters and digits, which would break the text matching.
const paragraphOne = "The sync loop starts from what the phone last acknowledged. " +
	"When the link comes back after an outage, the server does not replay every event it produced in the meantime. " +
	"It looks at the newest state of the current thread, compares it with the acknowledged state, and sends the difference, newest text first. " +
	"That keeps the first packets after an outage useful: the owner sees where the agent is now, not where it was a minute ago. " +
	"Older messages follow when the newest ones are safe on the phone.\n\n"

const paragraphTwo = "This paragraph is deliberately long, because long paragraphs are where the current server holds text back. " +
	"The server buffers an assistant message until a paragraph or a code fence is complete, so a client on a perfect link still waits for the end of the paragraph before it sees any of it. " +
	"On a terrible link that wait adds to every retransmission and every stall of the TCP connection under the WebSocket. " +
	"The lab measures the lag from the moment this fake agent produced each character, not from the moment the server published it, so that buffering counts against the transport like any other delay. " +
	"A sentence that took ten seconds to arrive is ten seconds old when the owner reads it, whatever the reason for the wait. " +
	"Next the agent asks for approval to write a file, because approvals are the other thing the owner must see quickly.\n\n"

const paragraphThree = "The approval came back, so the agent carries on. " +
	"Here is the image the agent made while it worked; it is small, but it still has to travel after the text. " +
	"Text the owner is waiting for always goes before images and before other threads. " +
	"The last part of this reply is a code block that the agent never closes, which is how a reply looks when a run stops in the middle of writing code.\n\n"

const soakParagraph = "The soak keeps this turn running for a long time on a bad link. " +
	"Every part adds a new paragraph, so the newest text keeps moving and the lab can see whether the client keeps up, falls behind, or stops receiving altogether. " +
	"A client that recovers after each stall shows a saw tooth of lag; a client that gives up shows a lag that only grows.\n\n"

const unclosedFence = "```\n" +
	"func recoverAfterOutage(ack State, newest State) []Packet {\n" +
	"\tdiff := newest.Since(ack)\n" +
	"\tpackets := diff.NewestTextFirst()\n" +
	"\tfor _, older := range diff.OlderMessages() {\n" +
	"\t\tpackets = append(packets, older)\n" +
	"\t}\n" +
	"\treturn packets\n" +
	"}\n" +
	"\n" +
	"func keepAlive(link Link) {\n" +
	"\tfor link.Up() {\n" +
	"\t\tlink.SendTiny(\"server alive, trying\")\n"
