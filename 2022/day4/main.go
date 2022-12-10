package main

import (
	"os"
	"strconv"
	"strings"
)

func main() {
	bytes, _ := os.ReadFile("input.txt")
	input := string(bytes)
	// For windows newlines
	strings.ReplaceAll(input, "\r\n", "\n")
	PartOne(input)
}

type assigment struct {
	from int
	to   int
}

func PartOne(input string) int {
	var overlaps int
	pairs := strings.Split(input, "\n")
	for i := range pairs {
		if len(pairs[i]) == 0 {
			continue
		}

		elf1, elf2 := parseAssigments(pairs[i])
		if elf1.from <= elf2.from && elf1.to >= elf2.to {
			overlaps += 1
			continue
		}
		if elf2.from <= elf1.from && elf2.to >= elf1.to {
			overlaps += 1
			continue
		}
	}

	return overlaps
}

func PartTwo(s string) int {
	var overlaps int
	pairs := strings.Split(s, "\n")
	for i := range pairs {
		if len(pairs[i]) == 0 {
			continue
		}

		elf1, elf2 := parseAssigments(pairs[i])
		if elf1.to >= elf2.from && elf1.to <= elf2.to {
			overlaps += 1
			continue
		}
		if elf2.to >= elf1.from && elf2.to <= elf1.to {
			overlaps += 1
			continue
		}
	}

	return overlaps
}

// parseAssigments parses string assigment like '5-7,7-9'
// into assiement struct tuple
func parseAssigments(s string) (assigment, assigment) {
	pairs := strings.Split(s, ",")

	e1 := strings.Split(pairs[0], "-")
	e1from, _ := strconv.Atoi(e1[0])
	e1to, _ := strconv.Atoi(e1[1])
	a1 := assigment{
		from: e1from,
		to:   e1to,
	}

	e2 := strings.Split(pairs[1], "-")
	e2from, _ := strconv.Atoi(e2[0])
	e2to, _ := strconv.Atoi(e2[1])
	a2 := assigment{
		from: e2from,
		to:   e2to,
	}

	return a1, a2
}
