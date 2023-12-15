package main

import (
	"fmt"
	"smolprog/logger"
	"strings"
)

func main() {
	channel := make(chan *logger.Message)
	defer close(channel)

	logrs := []logger.Logger{
		&logger.Network{},
		&logger.Cpu{},
		&logger.Memory{},
		&logger.Volume{},
		&logger.Sessions{},
		&logger.Battery{},
		&logger.Date{},
	}

	values := make([]string, len(logrs))

	for slot, logr := range logrs {
		go logr.Run(slot, channel)
	}

	for msg := range channel {
		values[msg.Slot] = msg.Value
		fmt.Println(strings.Join(values, ""))
	}
}
