package blocks

import (
	"os"
	"time"
)

var XDG_RUNTIME_DIR = os.Getenv("XDG_RUNTIME_DIR")

const SEP = "<Fn=1><Fg=#373737></Fg></Fn>"

type Message struct {
	Slot  int
	Value string
}

type Block interface{ Run(int, chan<- *Message) }

type IntervalBlock interface {
	Value() string
	Interval() time.Duration
}

func IntervalRunner(blk IntervalBlock, slot int, ch chan<- *Message) {
	msg := Message{Slot: slot}
	for {
		if msg.Value = blk.Value(); len(msg.Value) != 0 {
			ch <- &msg
		}
		time.Sleep(blk.Interval())
	}
}
