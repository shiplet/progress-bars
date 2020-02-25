package main

import (
	"bufio"
	"fmt"
	"math"
	"math/rand"
	"os"
	"strings"
	"sync/atomic"
	"time"
)

const COUNT int = 30
const RANGE float64 = 100
const SCALE float64 = 1.0

func outstr(index float64, total float64, scale float64) string {
	width := math.Round(total * scale)
	current := int(math.Round(((index / total) * 100.0) * scale))
	percentage := int(math.Round((index/total) * 100.0))
	return fmt.Sprintf("[%s>%s] %d%%",
		strings.Repeat("=", current),
		strings.Repeat(" ", int(width) - current),
		percentage)
}

func check_complete(progress_bars [COUNT]float64) bool {
	complete := true
	for _, v := range progress_bars {
		if v < RANGE {
			complete = false
		}
	}
	return complete
}

func get_incomplete_bars(progress_bars [COUNT]float64) []int {
	var incomplete []int
	for i, v := range progress_bars {
		if v < RANGE {
			incomplete = append(incomplete, i)
		}
	}
	return incomplete
}

func generator(incomplete_bars chan<- [COUNT]float64) {
	all_progress_bars := [COUNT]float64{}
	incomplete_bars <- all_progress_bars
}

func random_increment(printed_bars chan<- [COUNT]float64, incomplete_bars <-chan [COUNT]float64) {
	for bar := range incomplete_bars {
		incomplete := get_incomplete_bars(bar)
		rnd_index := rand.Intn(len(incomplete))
		bar[incomplete[rnd_index]]++

		printed_bars <- bar
	}
	close(printed_bars)
}

func get_increment(count <-chan int) {
	total := 0
	for ch := range count {
		total += ch
	}
}

func print_progress_bars(printed_bars <-chan [COUNT]float64, all_bars chan<- [COUNT]float64) {
	start := time.Now()
	var ops uint64
	b := bufio.NewWriter(os.Stdout)

	for ch := range printed_bars {
		atomic.AddUint64(&ops, 1)
		fmt.Fprint(b,"\u001b[1000D")
		fmt.Fprintf(b,"\u001b[%dA", COUNT)
		for _, bar := range ch {
			fmt.Fprintln(b, outstr(float64(bar), RANGE, SCALE))
		}
		if !check_complete(ch) {
			all_bars <- ch
		} else {
			close(all_bars)
			b.Flush()
			fmt.Printf("Time: %fs\n", time.Since(start).Seconds())
			fmt.Printf("Count: %d\n", ops)
			fmt.Printf("Time per op: %fs\n", time.Since(start).Seconds() / float64(ops))
		}
	}
}

func main() {
	incomplete_bars := make(chan [COUNT]float64)
	printed_bars := make(chan [COUNT]float64)

	go generator(incomplete_bars)
	go random_increment(printed_bars, incomplete_bars)

	print_progress_bars(printed_bars, incomplete_bars)
}
