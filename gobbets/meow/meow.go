package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"log"
)

func main() {
	log.SetFlags(log.Lshortfile)
	if len(os.Args) < 2 {
		log.Fatal("Specify a filename")
	}
	filename := os.Args[1]
	bytes, err := ioutil.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}
	text := string(bytes)
	fmt.Print(text)
}
