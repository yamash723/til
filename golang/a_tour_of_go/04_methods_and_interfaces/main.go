package main

import (
	"fmt"
	"math"
)

type Vertex struct {
	X, Y float64
}

// Methods
func (v Vertex) Abs() float64 {
	return math.Sqrt(v.X*v.X + v.Y*v.Y)
}

type MyFloat float64

func (f MyFloat) Abs() float64 {
	if f < 0 {
		return float64(-f)
	}
	return float64(f)
}

// Pointer receivers
func (v *Vertex) Scale(f float64) {
	v.X = v.X * f
	v.Y = v.Y * f
}

// Interface
type I interface {
	M()
}

type T struct {
	S string
}

func (t *T) M() {
	if t == nil {
		fmt.Println("<nil>")
		return
	}
	fmt.Println(t.S)
}

func main() {
	{
		// Methods
		v := Vertex{3, 4}
		fmt.Println(v.Abs())

		f := MyFloat(-math.Sqrt2)
		fmt.Println(f.Abs())
	}
	{
		// Pointer receivers
		v := Vertex{3, 4}
		v.Scale(10)
		fmt.Println(v.Abs())
	}
	{
		// Interface
		var i I
		var t *T
		i = t
		fmt.Printf("(%v, %T)\n", i, i)
		i.M()

		var x interface{}

		x = 41
		fmt.Printf("(%v, %T)\n", x, x)
		x = "string"
		fmt.Printf("(%v, %T)\n", x, x)
	}
}
