package main

import (
	"fmt"
	"math"
	"math/rand"
)

// can be omitted type annotation when arguments has same type.
// func add(x, y int) int {
func add(x int, y int) int {
	return x + y
}

func main() {
	fmt.Println("Hello, 世界")
	fmt.Println("My favorite number is", rand.Intn(10))
	fmt.Printf("Now you have %g problems", math.Sqrt(7))

	// error
	// fmt.Println(math.pi)
	fmt.Println(math.Pi)
}
