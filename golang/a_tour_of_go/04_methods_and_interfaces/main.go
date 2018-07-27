package main

import (
	"fmt"
	"image"
	"io"
	"math"
	"strings"
	"time"
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

// Type switches
func doSwitch(i interface{}) {
	switch v := i.(type) {
	case int:
		fmt.Printf("Twice %v is %v\n", v, v*2)
	case string:
		fmt.Printf("%q is %v bytes long\n", v, len(v))
	default:
		fmt.Printf("I don't know about type %T!\n", v)
	}
}

// Stringers
type Person struct {
	Name string
	Age  int
}

func (p Person) String() string {
	return fmt.Sprintf("%v (%v years)", p.Name, p.Age)
}

// Errors
type MyError struct {
	When time.Time
	What string
}

func (e *MyError) Error() string {
	return fmt.Sprintf("at %v, %s", e.When, e.What)
}

func run() error {
	return &MyError{
		time.Now(),
		"it didn't work",
	}
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
	{
		// Type assertions
		var i interface{} = "Hello World"
		s := i.(string)
		fmt.Println(s)

		s, ok := i.(string)
		fmt.Println(s, ok)

		f, ok := i.(float64)
		fmt.Println(f, ok)

		// error
		// f := i.(float64)
		// fmt.Println(f)
	}
	{
		// Type switches
		doSwitch(21)
		doSwitch("hello")
		doSwitch(true)
	}
	{
		// Stringers
		a := Person{"Arthur Dent", 42}
		z := Person{"Zaphod Beeblebrox", 9001}
		fmt.Println(a, z)
	}
	{
		// Errors
		if err := run(); err != nil {
			fmt.Println(err)
		}
	}
	{
		// Reader
		r := strings.NewReader("Hello, Reader!")
		b := make([]byte, 8)
		for {
			n, err := r.Read(b)
			fmt.Printf("n = %v err = %v b = %v\n", n, err, b)
			fmt.Printf("b[:n] = %q\n", b[:n])
			if err == io.EOF {
				break
			}
		}
	}
	{
		// Images
		m := image.NewRGBA(image.Rect(0, 0, 100, 100))
		fmt.Println(m.Bounds())
		fmt.Println(m.At(0, 0).RGBA())
	}
}
