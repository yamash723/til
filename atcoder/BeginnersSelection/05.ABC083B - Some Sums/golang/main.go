package main

import "fmt"

func main() {
	var max, a, b int
	fmt.Scan(&max)
	fmt.Scan(&a)
	fmt.Scan(&b)

	totalSum := 0
	for n := 1; n <= max; n++ {
		j := n
		sum := 0

		for j != 0 {
			sum += j % 10
			j = j / 10
		}

		if a <= sum && sum <= b {
			totalSum += n
		}
	}

	fmt.Println(totalSum)
}
