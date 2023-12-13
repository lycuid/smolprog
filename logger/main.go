package logger

import (
	. "smolprog/utils"
	"time"
)

type Logger interface{}

type FifoLogger interface {
	Logger
	FilePath() string
	Default() string
	Fmt(string) string
}

type Message struct {
	Position int
	Value    string
}

type Runner func(int, chan<- Message)

type IntervalLogger interface {
	Logger
	Value() string
	Interval() time.Duration
}

func IntervalRunner(obj IntervalLogger) Runner {
	return func(position int, channel chan<- Message) {
		for {
			if value := obj.Value(); len(value) != 0 {
				channel <- Message{position, value}
			}
			time.Sleep(obj.Interval())
		}
	}
}

func FifoRunner(obj FifoLogger) Runner {
	return func(position int, channel chan<- Message) {
		channel <- Message{position, obj.Fmt(obj.Default())}
		for {
			if value, err := FirstLineOf(obj.FilePath()); err == nil {
				channel <- Message{position, obj.Fmt(value)}
			}
		}
	}
}
