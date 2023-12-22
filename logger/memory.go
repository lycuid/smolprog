package logger

import (
	"fmt"
	"os/exec"
	. "smolprog/utils"
	"time"
)

type Memory struct{}

func (mem *Memory) Run(slot int, channel chan<- *Message) {
	IntervalRunner(mem, slot, channel)
}

func (_ *Memory) Interval() time.Duration {
	return time.Second
}

func (mem *Memory) Value() string {
	return SEP + "<BtnL=notify_max_mem>  " + mem.calculate() + "  </BtnL>"
}

func formatted(usage float32) (string, string) {
	if usage >= 1000 {
		return fmt.Sprintf("%.2f", usage/1024), "GiB"
	}
	return fmt.Sprintf("%4d", int(usage)), "Mib"
}

func (_ *Memory) calculate() string {
	var (
		lines        [][]string
		vals         []float32
		usage, total float32
	)

	cmd := exec.Command("free")
	if output, err := cmd.Output(); err == nil {
		lines = Filter(Map(Lines(string(output)), Words), func(ws []string) bool {
			return len(ws) > 0 && ws[0] == "Mem:"
		})
	}
	if len(lines) == 0 {
		goto DEFAULT
	}

	vals = Map[string, float32](lines[0][1:], Number)
	usage, total = (vals[1]+vals[3])/1024, vals[0]/1024

	switch value, unit := formatted(usage); {
	case InRange(usage, 0, total*.24):
		return fmt.Sprintf("  %s %s", value, unit)
	case InRange(usage, total*.25, total*.66):
		return fmt.Sprintf("  <Fg=#ffdd59>%s</Fg> %s", value, unit)
	case usage > total*.66:
		return fmt.Sprintf("  <Fg=#cc6666>%s</Fg> %s", value, unit)
	}

DEFAULT:
	return "  ?"
}
