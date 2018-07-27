package main

import (
	"fmt"
	"sync"
	"time"
)

// Goroutines
func say(s string) {
	for i := 0; i < 5; i++ {
		time.Sleep(100 * time.Microsecond)
		fmt.Println(s)
	}
}

// Channels
func sum(s []int, c chan int) {
	sum := 0
	for _, v := range s {
		sum += v
	}
	c <- sum
}

// Range and Close
func fibonacci(n int, c chan int) {
	x, y := 0, 1
	for i := 0; i < n; i++ {
		c <- x
		x, y = y, x+y
	}
	close(c)
}

// Select
func fibonacciS(c, quit chan int) {
	x, y := 0, 1
	for {
		select {
		case c <- x:
			x, y = y, x+y
		case <-quit:
			fmt.Println("quit")
			return
		}
	}
}

// sync.Mutex
type SafeCounter struct {
	v   map[string]int
	mux sync.Mutex
}

func (c *SafeCounter) Inc(key string) {
	c.mux.Lock()
	c.v[key]++
	c.mux.Unlock()
}

func (c *SafeCounter) Value(key string) int {
	c.mux.Lock()
	defer c.mux.Unlock()
	return c.v[key]
}

func main() {
	{
		// Goroutines
		go say("world")
		say("hello")
	}
	{
		// Channels
		s := []int{7, 2, 8, -9, 4, 0}
		c := make(chan int)
		go sum(s[:len(s)/2], c)
		go sum(s[len(s)/2:], c)
		x, y := <-c, <-c

		fmt.Println(x, y, x+y)
	}
	{
		// Buffer Channels

		ch := make(chan int, 2)
		ch <- 1
		ch <- 2
		// ch <- 3 // deadlock
		fmt.Println("ch 1: ", <-ch)
		fmt.Println("ch 2: ", <-ch)
	}
	{
		// Range and Close
		fmt.Println("----- fibonacci -----")
		c := make(chan int, 10)
		go fibonacci(cap(c), c)
		for i := range c {
			fmt.Println(i)
		}
	}
	{
		// Select
		fmt.Println("----- fibonacci with select -----")
		c := make(chan int)
		quit := make(chan int)
		go func() {
			for i := 0; i < 10; i++ {
				fmt.Println(<-c)
			}
			quit <- 0
		}()
		fibonacciS(c, quit)

		tick := time.Tick(100 * time.Millisecond)
		boom := time.After(500 * time.Millisecond)
	INFLOOP:
		for {
			select {
			case <-tick:
				fmt.Println("tick.")
			case <-boom:
				fmt.Println("BOOM!")
				break INFLOOP
			default:
				fmt.Println("    .")
				time.Sleep(50 * time.Millisecond)
			}
		}
	}
	{
		// sync.Mutex
		c := SafeCounter{v: make(map[string]int)}
		for i := 0; i < 1000; i++ {
			go c.Inc("somekey")
		}

		time.Sleep(time.Second)
		fmt.Println(c.Value("somekey"))
	}
}
