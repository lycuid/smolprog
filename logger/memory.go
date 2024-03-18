package logger

import (
	"bufio"
	"fmt"
	"os"
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

func formatted(usage float64) (string, string) {
	if usage >= 1000 {
		return fmt.Sprintf("%.2f", usage/1024), "GiB"
	}
	return fmt.Sprintf("%4d", int(usage)), "Mib"
}

func (_ *Memory) calculate() string {
	var usage, total float64

	file, err := os.Open("/proc/meminfo")
	if err != nil {
		goto DEFAULT
	}
	defer file.Close()

	for f, scanner := 0b1111, bufio.NewScanner(file); f > 0 && scanner.Scan(); {
		line := scanner.Text()
		switch {
		case StartsWith(line, "MemTotal:"):
			f &= ^(1 << 0)
			total += Float(Words(line)[1])
		case StartsWith(line, "MemFree:"):
			f &= ^(1 << 1)
			total -= Float(Words(line)[1])
		case StartsWith(line, "Buffers:"):
			f &= ^(1 << 2)
			total -= Float(Words(line)[1])
		case StartsWith(line, "Cached:"):
			f &= ^(1 << 3)
			total -= Float(Words(line)[1])
		}
	}
	usage = total / 1024.

	switch value, unit := formatted(usage); {
	case InRange(usage, 0, total*.25):
		return fmt.Sprintf("  %s %s", value, unit)
	case InRange(usage, total*.25, total*.66):
		return fmt.Sprintf("  <Fg=#ffdd59>%s</Fg> %s", value, unit)
	case usage > total*.66:
		return fmt.Sprintf("  <Fg=#cc6666>%s</Fg> %s", value, unit)
	}

DEFAULT:
	return "  ?"
}
