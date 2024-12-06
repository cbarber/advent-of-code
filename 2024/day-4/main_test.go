package main

import (
	"testing"
)

func TestPart1(t *testing.T) {
	input := `
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
`
	grid, err := ParseInput([]byte(input))
	if err != nil {
		t.Fatalf("failed to parse input: %+v", err)
	}
	result := Part1(grid)

	if result != 18 {
		t.Fatalf("actual %d did not equal expected 18: ", result)
	}
}

func TestPart2(t *testing.T) {
	input := `
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
`
	grid, err := ParseInput([]byte(input))
	if err != nil {
		t.Fatalf("failed to parse input: %+v", err)
	}
	result := Part2(grid)

	if result != 9 {
		t.Fatalf("actual %d did not equal expected 9: ", result)
	}
}
