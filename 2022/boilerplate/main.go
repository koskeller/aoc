package main

import (
	"os"
	"strings"
)

func main() {
	bytes, _ := os.ReadFile("input.txt")
	input := string(bytes)
	// For windows newlines
	strings.ReplaceAll(input, "\r\n", "\n")
	PartOne(input)
}

func PartOne(input string) {
}
