package main

import (
	"github.com/stephensli/advent-of-code-2021/helpers/aoc"
	"github.com/stephensli/advent-of-code-2021/helpers/file"
)

func solution(input []string) int {
	return len(input)
}

func main() {
	defer aoc.Setup(2015, 99)()

	input := file.ToTextLines("./input.txt")

	answer := solution(input)

	aoc.PrintAnswer(1, answer)
	aoc.PrintAnswer(2, answer+1)
}
