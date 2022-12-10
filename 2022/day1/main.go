package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	bytes, _ := os.ReadFile("input.txt")

	PartOne(string(bytes))
	PartTwo(string(bytes))
}

func PartOne(s string) {
	// For windows newlines
	strings.ReplaceAll(s, "\r\n", "\n")

	var calories []int
	for _, elf := range strings.Split(s, "\n\n") {
		var sum int
		for _, cal := range strings.Split(elf, "\n") {
			n, _ := strconv.Atoi(cal)
			sum += n
		}
		calories = append(calories, sum)
	}

	sort.Slice(calories, func(i, j int) bool {
		return calories[i] > calories[j]
	})

	var total int
	for _, v := range calories[:3] {
		total += v
	}

	fmt.Printf("The elf is carrying %v calories\n", total)
}

func PartTwo(s string) {
	// For windows newlines
	strings.ReplaceAll(s, "\r\n", "\n")

	var max int
	for _, elf := range strings.Split(s, "\n\n") {
		var sum int
		for _, cal := range strings.Split(elf, "\n") {
			value, _ := strconv.Atoi(cal)
			sum += value
		}
		if sum > max {
			max = sum
		}
	}

	fmt.Printf("Top three elves are carrying %v calories\n", max)
}
