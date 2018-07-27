package main

import (
	"fmt"

	"golang.org/x/tour/tree"
)

// Walk walks the tree t sending all values
// from the tree to the channel ch.
func Walk(t *tree.Tree, ch chan int) {
	var waker func(t *tree.Tree, ch chan int)
	waker = func(t *tree.Tree, ch chan int) {
		if t != nil {
			waker(t.Left, ch)
			ch <- t.Value
			waker(t.Right, ch)
		}
	}

	waker(t, ch)
	close(ch)
}

// Same determines whether the trees
// t1 and t2 contain the same values.
func Same(t1, t2 *tree.Tree) bool {
	t1ch, t2ch := make(chan int), make(chan int)

	go Walk(t1, t1ch)
	go Walk(t2, t2ch)

	for {
		t1v, t1ok := <-t1ch
		t2v, t2ok := <-t2ch

		if !t1ok && !t2ok {
			return true
		}

		if t1v != t2v {
			return false
		}
	}
}

func main() {
	ch := make(chan int)
	go Walk(tree.New(1), ch)
	for {
		v, ok := <-ch
		if !ok {
			break
		}

		fmt.Println(v)
	}

	if Same(tree.New(1), tree.New(1)) {
		fmt.Println("Same(tree.New(1), tree.New(1)) : is true")
	}

	if !Same(tree.New(1), tree.New(2)) {
		fmt.Println("Same(tree.New(1), tree.New(2)) : is false")
	}
}
