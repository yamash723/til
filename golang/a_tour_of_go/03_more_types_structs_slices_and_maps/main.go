package main

import (
	"fmt"
	"math"
)

func main() {
	{
		fmt.Println("---------- Pointers ----------")
		i, j := 42, 2701

		p := &i
		fmt.Println("Pointer address: ", p)
		fmt.Println("Value: ", *p)
		*p = 21
		fmt.Println("New value: ", *p)

		p = &j
		fmt.Println("Pointer address: ", p)
		fmt.Println("Value: ", *p)
		*p = *p / 37
		fmt.Println("New value: ", *p)
	}

	{
		fmt.Println("---------- Structs ----------")
		v1 := Vertex{1, 2}
		v2 := Vertex{X: 1}
		v3 := Vertex{}
		p := &Vertex{1, 2}
		fmt.Println(v1, v2, v3, p)
	}

	{
		fmt.Println("---------- Arrays ----------")
		var a [2]string
		a[0] = "Hello"
		a[1] = "World"
		fmt.Println(a[0], a[1])

		primes := [6]int{2, 3, 5, 7, 11, 13}
		fmt.Println(primes)
	}

	{
		fmt.Println("---------- Slices ----------")
		primes := [6]int{2, 3, 5, 7, 11, 13}

		s := primes[1:4]
		fmt.Println(s)

		names := [4]string{
			"John",
			"Paul",
			"George",
			"Ringo",
		}
		fmt.Println(names)

		a := names[0:2]
		b := names[1:3]
		fmt.Println(a, b)

		b[0] = "XXX"
		fmt.Println(a, b)
		fmt.Println(names)

		// literal
		q := []int{2, 3, 5, 7, 11, 13}
		fmt.Println(q)

		r := []bool{true, false}
		fmt.Println(r)

		st := []struct {
			i int
			b bool
		}{
			{2, true},
			{3, false},
			{5, true},
		}
		fmt.Println(st)

		// length & capacity
		sc := []int{2, 3, 5, 7, 11, 13}
		printSlice("Len a", sc)
		sc = s[:0]
		printSlice("Len b", sc)
		sc = s[:4]
		printSlice("Len c", sc)
		sc = s[2:]
		printSlice("Len d", sc)

		// make
		ma := make([]int, 5)
		printSlice("Make a", ma)
		mb := make([]int, 0, 5)
		printSlice("Make b", mb)
		mc := mb[:2]
		printSlice("Make c", mc)
		md := mc[2:5]
		printSlice("Make d", md)

		// append
		var sa []int
		printSlice("Append init", sa)
		sa = append(sa, 0)
		printSlice("Append a", sa)
		sa = append(sa, 1)
		printSlice("Append b", sa)
		sa = append(sa, 2, 3, 4, 5)
		printSlice("Append c", sa)
	}

	{
		fmt.Println("---------- Range ----------")
		pow := []int{1, 2, 4, 8, 16, 32, 64, 128}
		for i, v := range pow {
			fmt.Printf("2**%d = %d\n", i, v)
		}
	}

	{
		fmt.Println("---------- Maps ----------")
		var m map[string]VertexF
		m = make(map[string]VertexF)
		m["Bell Labs"] = VertexF{
			40.68433, -74.39967,
		}
		fmt.Println(m["Bell Labs"])

		var mp = map[string]VertexF{
			"Bell Labs": VertexF{
				40.68433, -74.39967,
			},
			"Google": VertexF{
				37.42202, -122.08408,
			},
			"Apple": {
				37.33224, -122.01319,
			},
		}

		delete(mp, "Google")

		if _, ok := mp["Google"]; !ok {
			fmt.Println("Google is deteled")
		}

		fmt.Println(mp)
	}

	{
		fmt.Println("---------- Function ----------")
		hypot := func(x, y float64) float64 {
			return math.Sqrt(x*x + y*y)
		}
		fmt.Println(hypot(5, 12))

		fmt.Println(compute(hypot))
		fmt.Println(compute(math.Pow))

		adder := func() func(int) int {
			sum := 0
			return func(x int) int {
				sum += x
				return sum
			}
		}

		pos, neg := adder(), adder()
		for i := 0; i < 10; i++ {
			fmt.Println(
				pos(i),
				neg(-2*i),
			)
		}
	}
}

type Vertex struct {
	X int
	Y int
}

type VertexF struct {
	X float32
	Y float32
}

func printSlice(s string, x []int) {
	fmt.Printf("%s len=%d cap=%d %v\n", s, len(x), cap(x), x)
}

func compute(fn func(float64, float64) float64) float64 {
	return fn(3, 4)
}
