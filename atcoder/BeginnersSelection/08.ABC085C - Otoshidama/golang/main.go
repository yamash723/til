package main

import "fmt"

func main() {
	var n, sum int

	fmt.Scan(&n, &sum)

	for x := 0; x < n+1; x++ {
		for y := 0; y < n-x+1; y++ {
			var z = n - x - y
			if (10000*x)+(5000*y)+(1000*z) == sum {
				fmt.Println(x, y, z)
				return
			}
		}
	}

	fmt.Println("-1 -1 -1")
}
