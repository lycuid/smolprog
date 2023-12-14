package logger

import "time"

type Message struct {
	Slot  int
	Value string
}

type Logger interface{ Run(int, chan<- *Message) }

type IntervalLogger interface {
	Value() string
	Interval() time.Duration
}

func IntervalRunner(interval_logger IntervalLogger, slot int, channel chan<- *Message) {
	for msg := (Message{Slot: slot}); ; time.Sleep(interval_logger.Interval()) {
		if msg.Value = interval_logger.Value(); len(msg.Value) != 0 {
			channel <- &msg
		}
	}
}
