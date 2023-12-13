package main

import (
	"fmt"
	"smolprog/logger"
	"strings"
)

func main() {
	channel := make(chan logger.Message)
	defer close(channel)

	runners := []logger.Runner{
		logger.IntervalRunner(&logger.Network{}),
		logger.IntervalRunner(&logger.Cpu{}),
		logger.IntervalRunner(&logger.Memory{}),
		logger.FifoRunner(&logger.Volume{}),
		logger.IntervalRunner(&logger.Sessions{}),
		logger.IntervalRunner(&logger.Date{}),
	}

	values := make([]string, len(runners))

	for i, runner := range runners {
		go runner(i, channel)
	}

	for msg := range channel {
		values[msg.Position] = msg.Value
		fmt.Println(strings.Join(values, ""))
	}
}
