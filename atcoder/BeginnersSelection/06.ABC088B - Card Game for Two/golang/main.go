package main

import (
	"fmt"
	"sort"
)

func main() {
	var max int
	fmt.Scan(&max)

	cards := scanNums(max)
	sort.Sort(sort.Reverse(sort.IntSlice(cards)))

	var alice, bob int
    for i := 0; i < len(cards); i++ {
		if i%2 == 0 {
			alice +=  cards[i]
		} else {
			bob +=  cards[i]
		}
	}
	
	fmt.Println(alice - bob)
}

func scanNums(len int) (nums []int) {
    var num int
    for i := 0; i < len; i++ {
        fmt.Scan(&num)
        nums = append(nums, num)
    }
    return
}
