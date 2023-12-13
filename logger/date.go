package logger

import (
	"fmt"
	"time"
)

const dateTemplate = "<Box:Top|Bottom|Left|Right=#089CAC><Bg=#171717><Fn=1> %s </Fn></Bg></Box>"

type Date struct{}

func (_ *Date) Interval() time.Duration {
	return time.Second
}

func (_ *Date) Value() string {
	return fmt.Sprintf(dateTemplate, time.Now().Format("Jan, Mon 02 15:04:05"))
}
