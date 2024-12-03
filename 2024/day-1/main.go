package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"

	parsec "github.com/prataprc/goparsec"
)

func main() {
	input := ReadInputFile()
	left, right, err := ParseInput(input)
	if err != nil {
		panic(fmt.Sprintf("failed to parse input: %v", err))
	}
	fmt.Printf("Part 1: %d", Part1(left, right))
}

func ReadInputFile() []byte {
	body, err := os.ReadFile("input")
	if err != nil {
		panic(fmt.Sprintf("unable to read file: %v", err))
	}
	return body
}

func ParseInput(input []byte) ([]int, []int, error) {
	s := parsec.NewScanner(input)

	line := parsec.And(
		nil,
		parsec.Int(),
		parsec.Int(),
	)
	lines := parsec.Many(nil, line)

	node, s := lines(s)

	var left []int
	var right []int

	for _, line := range node.([]parsec.ParsecNode) {
		values := line.([]parsec.ParsecNode)
		l := values[0].(*parsec.Terminal).Value
		r := values[1].(*parsec.Terminal).Value

		l_int, err := strconv.Atoi(l)
		if err != nil {
			return nil, nil, err
		}
		left = append(left, l_int)

		r_int, err := strconv.Atoi(r)
		if err != nil {
			return nil, nil, err
		}
		right = append(right, r_int)
	}

	return left, right, nil
}

func Part1(left []int, right []int) int {
	sort.Ints(left)
	sort.Ints(right)

	total := 0

	for i, l := range left {
		r := right[i]
		diff := l - r
		if diff < 0 {
			diff = -diff
		}
		total += diff
	}

	return total
}
