package main

import (
	"fmt"
)


func fizzbuzz(i int) (string) {
	if i % 3 == 0 && i % 5 == 0 {
		return "fizzbuzz"
	} else if i % 3 == 0 {
		return "fizz"
	} else if i % 5 == 0 {
		return "buzz"
	}
	return fmt.Sprintf("%d", i)
}

func main() {
	for i := 0; i <= 100; i++ {
		fmt.Println(fizzbuzz(i))
	}
}
