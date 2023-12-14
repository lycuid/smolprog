package logger

import (
	"fmt"
	"time"
)

type Date struct{}

func (date *Date) Run(slot int, channel chan<- *Message) {
	IntervalRunner(date, slot, channel)
}

func (_ *Date) Interval() time.Duration {
	return time.Second
}

func (_ *Date) Value() string {
	return fmt.Sprintf(
		"<Box:Top|Bottom|Left|Right=#089CAC>"+
			"<Bg=#171717>"+
			"<Fn=1>"+
			" %s "+
			"</Fn>"+
			"</Bg>"+
			"</Box>",
		time.Now().Format("Jan, Mon 02 15:04:05"),
	)
}
