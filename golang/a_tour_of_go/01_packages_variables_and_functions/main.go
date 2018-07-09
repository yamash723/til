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

// can be return multiple results
func swap(x, y string) (string, string) {
    return x, y
}

// Naked return. 
func split(sum int) (x, y int) {
	x = sum * 4 /9
	y = sum - x
	return // can be use blank return
}



func main() {
	fmt.Println("Hello, 世界")
	fmt.Println("My favorite number is", rand.Intn(10))
	fmt.Printf("Now you have %g problems", math.Sqrt(7))

	// error
	// fmt.Println(math.pi)
	fmt.Println(math.Pi)

	a, b := swap("Hello", "World")
	fmt.Println(a, b)

	fmt.Println(split(17))
	
	{
		var i = 1
		var x, y, z = "A", 1, true
		// i := 1
		// x, y, z := "A", 1, true
		fmt.Println(i, x, y, z)
	}

	// zero values
	{
		var i int
		var f float64
		var b bool
		var s string
		fmt.Printf("%v %v %v %q\n", i, f, b, s)
	}

	// type conversions
	{
		var x, y int = 3, 4
		var f float64 = math.Sqrt(float64(x*x + y*y))
		var z uint = uint(f)
		fmt.Println(x, y, z)
	}

	// constants
	{
		const Truth = true
		fmt.Println("Go rules?", Truth)
	}
}
