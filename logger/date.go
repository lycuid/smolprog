package logger

import "time"

type Date struct{}

func (date *Date) Run(slot int, channel chan<- *Message) {
	IntervalRunner(date, slot, channel)
}

func (_ *Date) Interval() time.Duration {
	return time.Second
}

func (_ *Date) Value() string {
	return SEP + "<Fn=1>  " + time.Now().Format("Jan, Mon 02 15:04:05") + " </Fn>"
}
