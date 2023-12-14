package logger

import (
	"fmt"
	"os/exec"
	. "smolprog/utils"
	"time"
)

type Sessions struct{}

func (ses *Sessions) Run(slot int, channel chan<- *Message) {
	IntervalRunner(ses, slot, channel)
}

func (_ *Sessions) Interval() time.Duration {
	return time.Second
}

func (ses *Sessions) Value() string {
	return "<BtnL=notify_tmux_ls> <Fg=#9b59b6>  " + ses.calculate() + "</Fg>  </BtnL>"
}

func (_ *Sessions) calculate() string {
	var lines []string

	cmd := exec.Command("tmux", "ls")
	if output, err := cmd.Output(); err == nil {
		lines = Lines(string(output))
	}

	return fmt.Sprint(len(lines))
}
