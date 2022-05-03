package main

import "fmt"

func contains(s []int, e int) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}

func main() {
	var n int
	fmt.Scan(&n)

	var cakes = make([]int, n)
	for i := 0; i < n; i++ {
		var c int
		fmt.Scan(&c)
		cakes[i] = c
	}

	var uniqueCakes []int
	for _, cake := range cakes {
		if !contains(uniqueCakes, cake) {
			uniqueCakes = append(uniqueCakes, cake)
		}
	}

	fmt.Println(len(uniqueCakes))
}
