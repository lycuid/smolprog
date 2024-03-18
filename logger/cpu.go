package logger

import (
	"fmt"
	. "smolprog/utils"
	"time"
)

type Cpu struct{ total, used int }

func (cpu *Cpu) Run(slot int, channel chan<- *Message) {
	IntervalRunner(cpu, slot, channel)
}

func (_ *Cpu) Interval() time.Duration {
	return time.Second
}

func (cpu *Cpu) Value() string {
	return SEP + "<BtnL=notify_max_cpu>  " + cpu.calculate() + "  </BtnL>"
}

func (cpu *Cpu) calculate() string {
	var (
		vals        []string
		total, used = cpu.total, cpu.used // old values
	)

	// invalid filepath should not crash the program.
	if line, err := FirstLineOf("/proc/stat"); err != nil {
		goto DEFAULT
	} else {
		vals = Words(line)[1:]
	}

	// new values.
	cpu.total = Sum(Map[string, int](vals[:7], Integer))
	cpu.used = Sum(Map[string, int](vals[:3], Integer))

	total, used = cpu.total-total, cpu.used-used

	switch usage := (used * 100) / Max(1, total); {
	case InRange(usage, 0, 24):
		return fmt.Sprintf("  %3d%%", usage)
	case InRange(usage, 25, 66):
		return fmt.Sprintf("  <Fg=#ffdd59>%3d</Fg>%%", usage)
	case usage > 66:
		return fmt.Sprintf("  <Fg=#cc6666>%3d</Fg>%%", usage)
	}

DEFAULT:
	return "  ?"
}
