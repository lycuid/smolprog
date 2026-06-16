package blocks

import (
	"fmt"
	"os"
	. "smolprog/util"
	"strconv"
	"strings"
	"time"
)

var (
	BAT_DIR = ""
	SYM     = []string{" ", " ", " ", " ", " "}
)

func init() {
	if entries, err := os.ReadDir("/sys/class/power_supply"); err == nil {
		for _, entry := range entries {
			if strings.HasPrefix(entry.Name(), "BAT") {
				BAT_DIR = "/sys/class/power_supply/" + entry.Name()
			}
		}
	}
}

type Battery struct{ sym_index int }

func (bat *Battery) Run(slot int, channel chan<- *Message) {
	IntervalRunner(bat, slot, channel)
}

func (*Battery) Interval() time.Duration {
	return time.Second
}

func (bat *Battery) Value() string {
	return SEP + "  " + bat.usage() + "  "
}

func (bat *Battery) usage() string {
	if capacity := bat.getCapacity(); capacity >= 0 {
		return fmt.Sprintf("%s %3d", bat.getStatus(capacity), capacity)
	}
	return "  ?"
}

func (*Battery) getCapacity() int {
	if capacity, err := FirstLineOf(BAT_DIR + "/capacity"); err == nil {
		if num, err := strconv.Atoi(capacity); err == nil {
			return num
		}
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
