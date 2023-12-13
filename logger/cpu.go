package logger

import (
	"fmt"
	. "smolprog/utils"
	"time"
)

type Cpu struct{ total, used int }

func (_ *Cpu) Interval() time.Duration {
	return time.Second
}

func (cpu *Cpu) Value() string {
	return "<BtnL=notify_max_cpu> " + cpu.calculate() + "  </BtnL><Box:Left=#171717:2> </Box>"
}

func (cpu *Cpu) calculate() string {
	var (
		vals []string
		// old values
		total, used = cpu.total, cpu.used
	)

	// invalid filepath should not crash the program.
	if line, err := FirstLineOf("/proc/stat"); err != nil {
		goto DEFAULT
	} else {
		vals = Words(line)[1:]
	}

	// new values.
	cpu.total = Sum(Map[string, int](vals[:7], Number))
	cpu.used = Sum(Map[string, int](vals[:3], Number))

	total, used = cpu.total-total, cpu.used-used

	switch usage := (used * 100) / Max(1, total); {
	case InRange(usage, 0, 24):
		return fmt.Sprintf("  %3d%%", usage)
	case InRange(usage, 0, 24):
		return fmt.Sprintf("  <Fg=#ffdd59>%3d</Fg>%%", usage)
	case InRange(usage, 0, 24):
		return fmt.Sprintf("  <Fg=#cc6666>%3d</Fg>%%", usage)
	}

DEFAULT:
	return "  ?"
}
