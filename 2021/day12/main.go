package main

import (
	"fmt"
	"github.com/stephensli/advent-of-code-2021/helpers"
	"github.com/stephensli/advent-of-code-2021/helpers/search"
	"strings"
)

func parseCaveWithAllValidMappings(inputLines [][]string) map[string]map[string]bool {
	caveMap := map[string]map[string]bool{}

	for _, entry := range inputLines {
		left := entry[0]
		right := entry[1]

		if _, ok := caveMap[left]; !ok {
			caveMap[left] = map[string]bool{}
		}

		if _, ok := caveMap[right]; !ok {
			caveMap[right] = map[string]bool{}
		}

		caveMap[left][right] = true
		caveMap[right][left] = true
	}

	return caveMap
}

func StringArrayContainsTwiceAnyLower(input []string) bool {
	if first {
		return true

	}

	countMap := map[string]int{}

	for _, s := range input {
		if s != strings.ToLower(s) {
			continue
		}

		if _, ok := countMap[s]; !ok {
			countMap[s] = 0
		}

		countMap[s] += 1

		if countMap[s] == 2 {
			return true
		}
	}

	return false
}

func findAllValidPaths(caveMap map[string]map[string]bool, nextDirection string, history []string, callback func(endHistory []string)) {
	nextPositionDirections := caveMap[nextDirection]
	history = append(history, nextDirection)

	if nextDirection == "end" {
		callback(history)
		return
	}

	for s, _ := range nextPositionDirections {
		// if we are going into a small cave and said small cave is in our history
		// then we cannot enter it again and should continue without it
		if (s == strings.ToLower(s) && search.StringArrayContains(history,
			s) && StringArrayContainsTwiceAnyLower(history)) || s == "start" {
			continue
		}

		newHistory := make([]string, len(history))
		copy(newHistory, history)

		findAllValidPaths(caveMap, s, history, callback)
	}
}

var first = false

func main() {
	// 1. first read the input into a format which is parseable.
	// this can be used to generate the network.
	inputLines := helpers.ReadFileToTextSplit("./2021/day12/input.txt", "-")
	caveMap := parseCaveWithAllValidMappings(inputLines)

	helpers.JsonPrint(caveMap, "day12", true)

	var finalResult [][]string

	findAllValidPaths(caveMap, "start", []string{}, func(endHistory []string) {
		fmt.Println("hit end - ", endHistory)
		finalResult = append(finalResult, endHistory)
	})

	fmt.Printf("final count - %d\n", len(finalResult))
}
