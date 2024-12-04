package main

import (
	"fmt"
	"os"
	"strconv"

	parsec "github.com/prataprc/goparsec"
)

func main() {
	input := ReadInputFile()
	reports, err := ParseInput(input)
	if err != nil {
		panic(fmt.Sprintf("failed to parse input: %v", err))
	}
	fmt.Printf("Part 1: %d", Part1(reports))
}

func ReadInputFile() []byte {
	body, err := os.ReadFile("input")
	if err != nil {
		panic(fmt.Sprintf("unable to read file: %v", err))
	}
	return body
}

func ParseInput(input []byte) ([][]int, error) {
	s := parsec.NewScanner(input)

	ast := parsec.NewAST("input", 100)

	newline := parsec.TokenExact("\n", "newline")

	report := ast.Kleene(
		"report",
		nil,
		parsec.Int(),
		parsec.TokenExact(" ", "space"),
	)
	reports := ast.Kleene("reports", nil, report, newline)

	root, _ := ast.Parsewith(reports, s)

	var result [][]int

	for _, line := range root.GetChildren() {
		values := line.GetChildren()
		var row []int
		for _, value := range values {
			i, _ := strconv.Atoi(value.GetValue())
			row = append(row, i)
		}
		result = append(result, row)
	}

	return result, nil
}

func Part1(reports [][]int) int {
	total := 0

	for _, row := range reports {
		if len(row) < 2 {
			continue
		}
		result := int(IsRowSafe(row))
		total += result
	}

	return total
}

func IsRowSafe(row []int) uint8 {
	prev_val := row[1] - row[0]

	if prev_val == 0 {
		return 0
	}

	if prev_val < -3 || prev_val == 0 || prev_val > 3 {
		return 0
	}

	for i := 2; i < len(row); i++ {
		val := row[i] - row[i-1]

		if (prev_val < 0 && val > 0) || (prev_val > 0 && val < 0) {
			return 0
		}

		if val < -3 || val == 0 || val > 3 {
			return 0
		}
	}

	return 1
}
