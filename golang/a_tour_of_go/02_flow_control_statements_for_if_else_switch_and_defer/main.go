package main

import (
	"fmt"
	"math"
	"runtime"
)

func main() {
	{
		sum := 0
		for i := 0; i < 10; i++ {
			sum++
		}
		fmt.Println(sum)
	}

	{
		if v := math.Pow(3, 2); v < 10 {
			fmt.Println(v)
		}
	}

	{
		fmt.Println(Sqrt(2))
	}

	{
		switch os := runtime.GOOS; os {
		case "darwin":
			fmt.Println("OS X.")
		case "linux":
			fmt.Println("Linux.")
		default:
			fmt.Printf("%s.", os)
		}
	}

	{
		v := 10
		switch {
		case v < 10:
			fmt.Println("low")
		case v >= 10:
			fmt.Println("high")
		}
	}

	defer fmt.Println("defer end.")

	{
		for i := 0; i < 10; i++ {
			defer fmt.Println(i)
		}
	}

	fmt.Println("end main.")
}

func Sqrt(x float64) float64 {
	z := 1.0
	for i := 0; i < 10; i++ {
		z -= (z*z - x) / (2 * z)
		fmt.Println(z)
	}
	return z
}
