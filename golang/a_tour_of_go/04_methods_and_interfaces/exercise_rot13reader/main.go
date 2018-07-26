package main

import (
	"io"
	"os"
	"strings"
)

type rot13Reader struct {
	r io.Reader
}

func rot13(b byte) byte {
	rot13Execute := func(m byte, alpStartPos byte) byte {
		return ((m - alpStartPos + 13) % 26) + alpStartPos
	}

	if 'a' <= b && b <= 'z' {
		return rot13Execute(b, 'a')
	} else if 'A' <= b && b <= 'Z' {
		return rot13Execute(b, 'A')
	}

	return b
}

func (r rot13Reader) Read(b []byte) (int, error) {
	n, err := r.r.Read(b)
	if err == nil {
		for i, m := range b {
			b[i] = rot13(m)
		}
	}

	return n, err
}

func main() {
	s := strings.NewReader("Lbh penpxrq gur pbqr!")
	r := rot13Reader{s}
	io.Copy(os.Stdout, &r)
}
