package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func readLines(fn string) ([]string, error) {
	f, err := os.Open(fn)
	if err != nil {
		return nil, err
	}
	defer f.Close()

	var lines []string
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	if scanner.Err() != nil {
		return nil, scanner.Err()
	}
	return lines, nil
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

func tok(pos int, line string, use_words bool) (string, int, int) {
	if line[pos] >= '0' && line[pos] <= '9' {
		return "num", int(line[pos] - '0'), 1
	}
	if !use_words {
		return "unknown", 0, 1
	}
	for tok, val := range tokens {
		if strings.HasPrefix(line[pos:], tok) {
			return "num", val, len(tok)
		}
	}
	return "unknown", 0, 1
}

func parseLine(line string, use_words bool) (int, int) {
	var first int
	var last int
	var first_found bool = false
	for i := 0; i < len(line); {
		tokType, tokVal, tokLen := tok(i, line, use_words)
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

func parseLines(lines []string, use_words bool) []int {
	var nums []int
	for _, line := range lines {
		first, last := parseLine(line, use_words)
		nums = append(nums, first+last)
	}
	return nums
}

func solve(lines []string, use_words bool) {
	nums := parseLines(lines, use_words)
	total := 0
	for _, num := range nums {
		total += num
	}
	fmt.Println(total)
}

func main() {
	lines, err := readLines(os.Args[1])
	if err != nil {
		fmt.Fprintln(os.Stderr, "readLines:", err)
	}
	solve(lines, false)
	solve(lines, true)
}
