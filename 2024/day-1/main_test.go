package main

import (
	"reflect"
	"testing"
)

func TestParseInput(t *testing.T) {
	input := `
3   4
4   3
2   5
1   3
3   9
3   3`

	left, right, err := ParseInput([]byte(input))
	if err != nil {
		t.Fatal(err)
	}

	expected_left := []int{3, 4, 2, 1, 3, 3}
	if !reflect.DeepEqual(left, expected_left) {
		t.Fatal("left did not equal expected_left", left, expected_left)
	}

	expected_right := []int{4, 3, 5, 3, 9, 3}
	if !reflect.DeepEqual(right, expected_right) {
		t.Fatal("left did not equal expected_left", right, expected_right)
	}
}

func TestPart1(t *testing.T) {
	left := []int{3, 4, 2, 1, 3, 3}
	right := []int{4, 3, 5, 3, 9, 3}

	result := Part1(left, right)

	if result != 11 {
		t.Fatalf("expected result %d to equal 11: ", result)
	}
}

func TestPart2(t *testing.T) {
	left := []int{3, 4, 2, 1, 3, 3}
	right := []int{4, 3, 5, 3, 9, 3}

	result := Part2(left, right)

	if result != 31 {
		t.Fatalf("expected result %d to equal 31: ", result)
	}
}
