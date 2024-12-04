package main

import (
	"reflect"
	"testing"
)

func TestParseInput(t *testing.T) {
	input := `
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`

	reports, err := ParseInput([]byte(input))
	if err != nil {
		t.Fatal(err)
	}

	expected_first := []int{7, 6, 4, 2, 1}
	if !reflect.DeepEqual(reports[0], expected_first) {
		t.Fatal("left did not equal expected_left", reports[0], expected_first)
	}

	expected_last := []int{1, 3, 6, 7, 9}
	if !reflect.DeepEqual(reports[5], expected_last) {
		t.Fatal("left did not equal expected_left", reports[5], expected_last)
	}
}

func TestPart1(t *testing.T) {
	reports := [][]int{
		{7, 6, 4, 2, 1},
		{1, 2, 7, 8, 9},
		{9, 7, 6, 2, 1},
		{1, 3, 2, 4, 5},
		{8, 6, 4, 4, 1},
		{1, 3, 6, 7, 9},
	}

	result := Part1(reports)

	if result != 2 {
		t.Fatalf("actual %d did not equal expected 2: ", result)
	}
}

func TestPart2(t *testing.T) {
	reports := [][]int{
		{7, 6, 4, 2, 1},
		{1, 2, 7, 8, 9},
		{9, 7, 6, 2, 1},
		{1, 3, 2, 4, 5},
		{8, 6, 4, 4, 1},
		{1, 3, 6, 7, 9},
	}

	result := Part2(reports)

	if result != 4 {
		t.Fatalf("actual %d did not equal expected 4: ", result)
	}
}
