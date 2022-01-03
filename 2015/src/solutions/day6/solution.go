package main

import (
	"github.com/stephensli/advent-of-code-2021/helpers/aoc"
	"github.com/stephensli/advent-of-code-2021/helpers/file"
)

type Position struct {
	X, Y int
}

type Action struct {
	From, To Position
	Toggle   bool
	OnOff    bool
}

func solution(input []Action, partOne bool) int {
	grid := NewGrid(1000)

	for _, action := range input {
		if action.Toggle {
			grid.ToggleLights(action.From, action.To, partOne)
		}

		if !action.Toggle {
			grid.TurnOnOffLights(action.OnOff, action.From, action.To, partOne)
		}
	}

	return grid.on
}

func main() {
	defer aoc.Setup(2015, 6)()

	input := file.ToTextLines("./input.txt")
	actions := parseInput(input)

	aoc.PrintAnswer(1, solution(actions, true))
	aoc.PrintAnswer(2, solution(actions, false))
}
