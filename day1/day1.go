package main

import (
	"bufio"
	"fmt"
	"os"
)

func readInput(fn string) []int {
	f, err := os.Open(fn)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	var nums []int
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		var first int
		var last int
		var first_found bool = false
		for _, r := range line {
			if r >= '0' && r <= '9' {
				if !first_found {
					first = int(r-'0') * 10
					first_found = true
				}
				last = int(r - '0')
			}
		}
		nums = append(nums, first+last)
	}
	return nums
}

var tokens = map[string]int{
	"one":   1,
	"two":   2,
	"three": 3,
	"four":  4,
	"five":  5,
	"six":   6,
	"seven": 7,
	"eight": 8,
	"nine":  9,
}

func tok(pos int, line string) (string, int, int) {
	if line[pos] >= '0' && line[pos] <= '9' {
		return "num", int(line[pos] - '0'), 1
	}
	for tok, val := range tokens {
		if len(line)-pos >= len(tok) && line[pos:pos+len(tok)] == tok {
			return "num", val, len(tok)
		}
	}
	return "unknown", 0, 1
}

func parseLine(line string) (int, int) {
	var first int
	var last int
	var first_found bool = false
	for i := 0; i < len(line); {
		tokType, tokVal, tokLen := tok(i, line)
		if tokType == "num" {
			if !first_found {
				first = tokVal * 10
				first_found = true
			}
			last = tokVal
		}
		i += tokLen
	}
	return first, last

}

func parseInput(fn string) []int {
	f, err := os.Open(fn)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	var nums []int
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		first, last := parseLine(line)
		nums = append(nums, first+last)
	}
	return nums
}

func solve(parser func(string) []int, fn string) {
	nums := parser(fn)
	total := 0
	for _, num := range nums {
		total += num
	}
	fmt.Println(total)
}

func main() {
	solve(readInput, os.Args[1])
	solve(parseInput, os.Args[1])
}
