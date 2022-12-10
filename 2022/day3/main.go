package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	bytes, _ := os.ReadFile("input.txt")
	PartOne(string(bytes))
}

func PartOne(input string) {
	var sum int
	rucksacks := strings.Split(input, "\n")
	for i := 3; i < len(rucksacks); i += 3 {
		m := map[int][]int{
			0: make([]int, 53),
			1: make([]int, 53),
			2: make([]int, 53),
		}

		for elf, rucksack := range rucksacks[i-3 : i] {
			for _, item := range rucksack {
				priority := getPriority(item)
				m[elf][priority] = priority
			}
		}

		for _, priority := range m[0] {
			if priority != 0 && m[1][priority] != 0 && m[2][priority] != 0 {
				sum += priority
				break
			}
		}
	}

	fmt.Println(sum)
}

func getPriority(r rune) int {
	if r >= 'a' {
		return int(r) - 'a' + 1 // so we get 1 to 26
	} else {
		return int(r) - 'A' + 27
	}
}
