package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
)

func main() {
	sc := NewScanner(os.Stdin)

	n := sc.Int()

	for i := 0; i < n; i++ {
		t := sc.Int()
		x := sc.Int()
		y := sc.Int()
		if t < (x+y) || t%2 != (x+y)%2 {
			fmt.Println("No")
			return
		}
	}

	fmt.Println("Yes")
}

type Scanner struct {
	*bufio.Scanner
}

func NewScanner(r io.Reader) *Scanner {
	s := bufio.NewScanner(r)
	s.Split(bufio.ScanWords)
	return &Scanner{s}
}

func (s *Scanner) UInt() uint64 {
	s.Scan()
	i, e := strconv.ParseUint(s.Text(), 10, 64)
	if e != nil {
		panic(e)
	}
	return i
}

func (s *Scanner) Int() int {
	return int(s.UInt())
}
