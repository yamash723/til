package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func toHalf(a []int) []int {
	var halfA []int
	for _, v := range a {
		if v%2 == 1 {
			return nil
		}
		halfA = append(halfA, v/2)
	}

	return halfA
}

var sc = bufio.NewScanner(os.Stdin)

func main() {
	if sc.Scan() {
		_ = sc.Text()
	}

	var line string
	if sc.Scan() {
		line = sc.Text()
	}

	var a []int
	for _, v := range strings.Split(line, " ") {
		var iv, _ = strconv.Atoi(v)
		a = append(a, iv)
	}

	cnt := 0
	for {
		a = toHalf(a)
		if a == nil {
			break
		}

		cnt += 1
	}

	fmt.Println(cnt)
}
