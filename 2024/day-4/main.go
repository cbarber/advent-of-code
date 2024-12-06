package main

import (
	"fmt"
	"os"
	"reflect"
	"strings"
)

func main() {
	input := ReadInputFile()
	result, err := ParseInput(input)
	if err != nil {
		panic(fmt.Sprintf("failed to parse input: %v", err))
	}
	fmt.Printf("Part 1: %d\n", Part1(result))
	fmt.Printf("Part 2: %d\n", Part2(result))
}

func ReadInputFile() []byte {
	body, err := os.ReadFile("input")
	if err != nil {
		panic(fmt.Sprintf("unable to read file: %v", err))
	}
	return body
}

func ParseInput(input []byte) ([][]rune, error) {
	var result [][]rune

	text := string(input)
	text = strings.TrimSpace(text)

	for _, line := range strings.Split(text, "\n") {
		result = append(result, []rune(line))
	}

	return result, nil
}

func Part1(grid [][]rune) int {
	total := 0

	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[i]); j++ {
			total += search(grid, i, j, "XMAS")
		}
	}

	return total
}

func search(grid [][]rune, i, j int, s string, delta ...int) int {
	if i < 0 || j < 0 || i >= len(grid) || j >= len(grid[i]) {
		return 0
	}

	if grid[i][j] != rune(s[0]) {
		return 0
	}

	if len(s) == 1 {
		return 1
	}

	next := s[1:]

	if len(delta) == 2 {
		i += delta[0]
		j += delta[1]

		return search(grid, i, j, next, delta...)
	}

	return search(grid, i-1, j-1, next, -1, -1) +
		search(grid, i-1, j, next, -1, 0) +
		search(grid, i-1, j+1, next, -1, 1) +
		search(grid, i, j-1, next, 0, -1) +
		search(grid, i, j+1, next, 0, 1) +
		search(grid, i+1, j-1, next, 1, -1) +
		search(grid, i+1, j, next, 1, 0) +
		search(grid, i+1, j+1, next, 1, 1)
}

func Part2(grid [][]rune) int {
	total := 0

	allowed := [][]rune{
		[]rune("MMSS"),
		[]rune("MSSM"),
		[]rune("SSMM"),
		[]rune("SMMS"),
	}

	for i := 1; i < len(grid)-1; i++ {
		for j := 1; j < len(grid[i])-1; j++ {
			if grid[i][j] != 'A' {
				continue
			}

			outer := []rune{grid[i-1][j-1], grid[i-1][j+1], grid[i+1][j+1], grid[i+1][j-1]}

			for _, allow := range allowed {
				if reflect.DeepEqual(outer, allow) {
					total += 1
					break
				}
			}
		}
	}

	return total
}
