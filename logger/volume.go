package logger

import . "smolprog/utils"

type Volume struct{}

func (vol *Volume) Run(slot int, channel chan<- *Message) {
	msg := Message{Slot: slot, Value: vol.Fmt("  ?")}

	for channel <- &msg; ; {
		if line, err := FirstLineOf(XDG_RUNTIME_DIR + "/pipe/volume"); err == nil {
			msg.Value = vol.Fmt(line)
			channel <- &msg
		}
	}
}

func (_ *Volume) Fmt(s string) string {
	return "<ScrlU:Shift=volume 5%+>" +
		"<ScrlD:Shift=volume 5%->" +
		"<ScrlU=volume 1%+>" +
		"<ScrlD=volume 1%->" +
		"<BtnL=volume toggle> " + s + "  </BtnL>" +
		"</ScrlD>" +
		"</ScrlU>" +
		"</ScrlD>" +
		"</ScrlU>" +
		"<Box:Left=#171717:2> </Box>"
}
