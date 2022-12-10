package main

import (
	"fmt"
	"os"
	"strings"
)

type move int

const (
	rock move = iota + 1
	paper
	scissors
)

type outcome int

const (
	loss outcome = 0
	draw outcome = 3
	win  outcome = 6
)

var winningMoves = map[move]move{
	rock:     paper,
	scissors: rock,
	paper:    scissors,
}

var loosingMoves = map[move]move{
	rock:     scissors,
	scissors: paper,
	paper:    rock,
}

func main() {
	bytes, _ := os.ReadFile("input.txt")
	input := string(bytes)
	// For windows newlines
	strings.ReplaceAll(input, "\r\n", "\n")
	PartOne(input)
}

func PartOne(input string) {
	var score int
	for _, round := range strings.Split(input, "\n") {
		if len(round) == 0 {
			continue
		}
		play := strings.Split(round, " ")

		theirsMove := parseMove(play[0])
		desiredOutcome := parseOutcome(play[1])
		ourMove := getDesiredHand(desiredOutcome, theirsMove)

		score += int(ourMove) + int(desiredOutcome)
	}

	fmt.Println(score)
}

// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
func parseOutcome(s string) outcome {
	switch s {
	case "X":
		return loss
	case "Y":
		return draw
	case "Z":
		return win
	default:
		panic(fmt.Sprintf("unknown outcome %s", s))
	}
}

// A for Rock, B for Paper, and C for Scissors
func parseMove(s string) move {
	switch s {
	case "A":
		return rock
	case "B":
		return paper
	case "C":
		return scissors
	default:
		panic(fmt.Sprintf("unknown move %s", s))
	}
}

func getDesiredHand(desiredOutcome outcome, theirsMove move) move {
	switch desiredOutcome {
	case draw:
		return theirsMove
	case loss:
		return loosingMoves[theirsMove]
	case win:
		return winningMoves[theirsMove]
	default:
		panic("unknown outcome")
	}
}

// Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
func getScore(theirMove, ourMove move) outcome {
	if theirMove == ourMove {
		return draw
	}

	if theirMove == rock && ourMove == scissors {
		return loss
	}

	if theirMove == scissors && ourMove == paper {
		return loss
	}

	if theirMove == paper && ourMove == rock {
		return loss
	}

	return win
}
