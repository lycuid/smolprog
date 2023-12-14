package main

import (
	"fmt"
	"smolprog/logger"
	"strings"
)

func main() {
	channel := make(chan *logger.Message)
	defer close(channel)

	loggers := []logger.Logger{
		&logger.Network{},
		&logger.Cpu{},
		&logger.Memory{},
		&logger.Volume{},
		&logger.Sessions{},
		&logger.Date{},
	}

	values := make([]string, len(loggers))

	for i, log := range loggers {
		go log.Run(i, channel)
	}

	for msg := range channel {
		values[msg.Slot] = msg.Value
		fmt.Println(strings.Join(values, ""))
	}
}
