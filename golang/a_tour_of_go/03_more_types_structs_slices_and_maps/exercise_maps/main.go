package main

import (
	"strings"

	"golang.org/x/tour/wc"
)

func WordCount(s string) map[string]int {
	wordCount := make(map[string]int)
	words := strings.Fields(s)

	for _, word := range words {
		wordCount[word] += 1
	}
	return wordCount
}

func main() {
	wc.Test(WordCount)
}
