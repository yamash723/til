package main

import "fmt"

// fibonacci is a function that returns
// a function that returns an int.
func fibonacci() func() int {
	var count, oneBefore, twoBefore int
	return func() int {
		var result int
		if count == 1 {
			result = 1
		} else {
			result = oneBefore + twoBefore
		}
		oneBefore = twoBefore
		twoBefore = result
		count++
		return result
	}
}

func main() {
	f := fibonacci()
	for i := 0; i < 10; i++ {
		fmt.Println(f())
	}
}
