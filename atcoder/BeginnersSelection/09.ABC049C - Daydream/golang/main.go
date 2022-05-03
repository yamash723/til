package main

import (
	"fmt"
	"strings"
)

func main() {
	var s string
	fmt.Scan(&s)

	words := []string{"dream", "erase", "dreamer", "eraser"}

LOOP:
	for {
		for _, word := range words {
			if strings.HasSuffix(s, word) {
				s = s[:len(s)-len(word)]
				continue LOOP
			}
		}

		break
	}

	if s == "" {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}
