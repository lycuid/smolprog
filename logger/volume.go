package logger

import "os"

type Volume struct{}

func (_ *Volume) FilePath() string {
	return os.Getenv("XDG_RUNTIME_DIR") + "/pipe/volume"
}

func (_ *Volume) Default() string {
	return "  ?"
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
