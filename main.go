package main

import (
	"fmt"
	"smolprog/blocks"
	"strings"
)

func main() {
	ch := make(chan *blocks.Message)
	defer close(ch)

	blks := []blocks.Block{
		&blocks.Network{},
		&blocks.Cpu{},
		&blocks.Memory{},
		&blocks.Volume{},
		&blocks.Sessions{},
		&blocks.Battery{},
		&blocks.Date{},
	}

	values := make([]string, len(blks))

	for slot, blk := range blks {
		go blk.Run(slot, ch)
	}

	for msg := range ch {
		values[msg.Slot] = msg.Value
		fmt.Println(strings.Join(values, ""))
	}
}
