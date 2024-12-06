package main

import (
	"reflect"
	"testing"
)

func TestParseInput(t *testing.T) {
	input := `
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
`

	instructions, err := ParseInput([]byte(input))
	if err != nil {
		t.Fatal(err)
	}

	expected := []Instruction{
		{operator: Multiply, left: 2, right: 4},
		{operator: Multiply, left: 5, right: 5},
		{operator: Multiply, left: 11, right: 8},
		{operator: Multiply, left: 8, right: 5},
	}
	if !reflect.DeepEqual(instructions, expected) {
		t.Fatal("instructions did not equal expected", instructions, expected)
	}
}

func TestPart1(t *testing.T) {
	instructions := []Instruction{
		{operator: Multiply, left: 2, right: 4},
		{operator: Multiply, left: 5, right: 5},
		{operator: Multiply, left: 11, right: 8},
		{operator: Multiply, left: 8, right: 5},
	}

	result := Part1(instructions)

	if result != 161 {
		t.Fatalf("actual %d did not equal expected 161: ", result)
	}
}
