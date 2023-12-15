package logger

import (
	"fmt"
	. "smolprog/utils"
	"time"
)

var (
	BAT_DIR = "/sys/class/power_supply/BAT0"
	SYM     = []string{" ", " ", " ", " ", " "}
)

type Battery struct{ sym_index int }

func (bat *Battery) Run(slot int, channel chan<- *Message) {
	IntervalRunner(bat, slot, channel)
}

func (_ *Battery) Interval() time.Duration {
	return time.Second
}

func (bat *Battery) Value() string {
	return " " + bat.calculate() + "  "
}

func (bat *Battery) calculate() string {
	if capacity := bat.getCapacity(); capacity >= 0 {
		return fmt.Sprintf("%s %3d", bat.getStatus(capacity), capacity)
	}
	return "  ?"
}

func (_ *Battery) getCapacity() int {
	if capacity, err := FirstLineOf(BAT_DIR + "/capacity"); err == nil {
		return Number[int](capacity)
	}
	return -1
}

func (bat *Battery) getStatus(capacity int) string {
	bat.sym_index = (bat.sym_index + 1) % len(SYM)

	switch status, _ := FirstLineOf(BAT_DIR + "/status"); status {
	case "Charging":
		return SYM[bat.sym_index]
	case "Discharging":
		return SYM[capacity*len(SYM)/101]
	}
	return SYM[len(SYM)-1]
}
