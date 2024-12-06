package main

import (
	"fmt"
	"os"
	"strconv"

	parsec "github.com/prataprc/goparsec"
)

func main() {
	input := ReadInputFile()
	result, err := ParseInput(input)
	if err != nil {
		panic(fmt.Sprintf("failed to parse input: %v", err))
	}
	fmt.Printf("Part 1: %d\n", Part1(result))
}

func ReadInputFile() []byte {
	body, err := os.ReadFile("input")
	if err != nil {
		panic(fmt.Sprintf("unable to read file: %v", err))
	}
	return body
}

type Operator int

const (
	Multiply Operator = iota + 1
)

type Instruction struct {
	operator Operator
	left     int
	right    int
}

func ParseInput(input []byte) ([]Instruction, error) {
	s := parsec.NewScanner(input)

	mapInstruction := func(nodes []parsec.ParsecNode) parsec.ParsecNode {
		left, _ := strconv.Atoi(nodes[1].(*parsec.Terminal).GetValue())
		right, _ := strconv.Atoi(nodes[3].(*parsec.Terminal).GetValue())
		return Instruction{
			operator: Multiply,
			left:     left,
			right:    right,
		}
	}
	number := parsec.TokenExact("[0-9]{1,3}", "number")

	instruction := parsec.And(
		mapInstruction,
		parsec.Atom("mul(", "open"),
		number,
		parsec.AtomExact(",", "comma"),
		number,
		parsec.AtomExact(")", "close"),
	)

	invalid := parsec.Token(`.|\s|$`, "invalid")

	maybeInstruction := parsec.OrdChoice(nil, instruction, invalid)

	instructions := parsec.ManyUntil(nil, maybeInstruction, parsec.End())

	root, next := instructions(s)

	if !next.Endof() {
		return nil, fmt.Errorf("failed to parse '%s' char number %d", input, next.GetCursor())
	}

	var result []Instruction

	for _, manyTokens := range root.([]parsec.ParsecNode) {
		for _, maybeInstruction := range manyTokens.([]parsec.ParsecNode) {
			switch j := maybeInstruction.(type) {
			case Instruction:
				result = append(result, j)
			}
		}
	}

	return result, nil
}

func Part1(instructions []Instruction) int {
	total := 0

	for _, instruction := range instructions {
		switch instruction.operator {
		case Multiply:
			total += instruction.left * instruction.right
		}
	}

	return total
}
