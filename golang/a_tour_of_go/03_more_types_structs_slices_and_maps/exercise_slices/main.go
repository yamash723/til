package main

import "golang.org/x/tour/pic"

func Pic(dx, dy int) [][]uint8 {
	slices := make([][]uint8, dy)
	for i := range slices {
		slice := make([]uint8, dx)

		for j := range slice {
			slice[j] = uint8((i + j) / 2)
		}

		slices[i] = slice
	}

	return slices
}

func main() {
	pic.Show(Pic)
}
