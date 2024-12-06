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
	fmt.Printf("Part 2: %d\n", Part2(result))
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
	Do
	Dont
)

type Instruction struct {
	operator Operator
	left     int
	right    int
}

func ParseInput(input []byte) ([]Instruction, error) {
	s := parsec.NewScanner(input)

	mapMultiply := func(nodes []parsec.ParsecNode) parsec.ParsecNode {
		left, _ := strconv.Atoi(nodes[1].(*parsec.Terminal).GetValue())
		right, _ := strconv.Atoi(nodes[3].(*parsec.Terminal).GetValue())
		return Instruction{
			operator: Multiply,
			left:     left,
			right:    right,
		}
	}
	number := parsec.TokenExact("[0-9]{1,3}", "number")

	multiply := parsec.And(
		mapMultiply,
		parsec.Atom("mul(", "open"),
		number,
		parsec.AtomExact(",", "comma"),
		number,
		parsec.AtomExact(")", "close"),
	)

	mapToggle := func(nodes []parsec.ParsecNode) parsec.ParsecNode {
		switch nodes[0].(*parsec.Terminal).GetName() {
		case "do":
			return Instruction{
				operator: Do,
				left:     0,
				right:    0,
			}
		case "dont":
			return Instruction{
				operator: Dont,
				left:     0,
				right:    0,
			}

		}
		return nil
	}

	toggle := parsec.OrdChoice(
		mapToggle,
		parsec.Atom("don't()", "dont"),
		parsec.Atom("do()", "do"),
	)

	invalid := parsec.Token(`.|\s|$`, "invalid")

	maybeInstruction := parsec.OrdChoice(nil, multiply, toggle, invalid)

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

func Part2(instructions []Instruction) int {
	total := 0
	enabled := true

	for _, instruction := range instructions {
		switch instruction.operator {
		case Multiply:
			if enabled {
				total += instruction.left * instruction.right
			}
		case Do:
			enabled = true
		case Dont:
			enabled = false
		}
	}

	return total
}
