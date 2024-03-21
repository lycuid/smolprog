package logger

import . "smolprog/utils"

type Volume struct{}

func (vol *Volume) Run(slot int, channel chan<- *Message) {
	default_value := vol.Fmt("  ?")
	msg := Message{Slot: slot, Value: default_value}

	for channel <- &msg; ; {
		if line, err := FirstLineOf(XDG_RUNTIME_DIR + "/pipe/volume"); err == nil {
			if len(line) > 0 {
				msg.Value = vol.Fmt(line)
			} else {
				msg.Value = default_value
			}
			channel <- &msg
		}
	}
}

func (_ *Volume) Fmt(s string) string {
	return SEP + "<ScrlU:Shift=volume 5%+>" +
		"<ScrlD:Shift=volume 5%->" +
		"<ScrlU=volume 1%+>" +
		"<ScrlD=volume 1%->" +
		"<BtnL=volume toggle>  " + s + "  </BtnL>" +
		"</ScrlD>" +
		"</ScrlU>" +
		"</ScrlD>" +
		"</ScrlU>"
}
