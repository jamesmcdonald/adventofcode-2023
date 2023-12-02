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

func parseLine(line string) (int, int) {
	var game int
	parts := strings.Split(line, ":")
	fmt.Sscanf(parts[0], "Game %d", &game)
	guesses := strings.Split(parts[1], ";")
	maxes := map[string]int{
		"red":   0,
		"green": 0,
		"blue":  0,
	}
	for _, guess := range guesses {
		colours := strings.Split(guess, ",")
		for _, colour := range colours {
			var name string
			var count int
			fmt.Sscanf(colour, "%d %s", &count, &name)
			if count > totals[name] {
				game = 0
			}
			if count > maxes[name] {
				maxes[name] = count
			}
		}
	}
	return game, maxes["red"] * maxes["green"] * maxes["blue"]
}

func parseLines(lines []string) (int, int) {
	var totalgame, totalpower int
	for _, line := range lines {
		game, power := parseLine(line)
		totalgame += game
		totalpower += power
	}
	return totalgame, totalpower
}

var totals = map[string]int{
	"red":   12,
	"green": 13,
	"blue":  14,
}

func main() {
	if len(os.Args) != 2 {
		fmt.Fprintf(os.Stderr, "Usage: %s <inputfile>\n", os.Args[0])
		os.Exit(1)
	}
	lines, err := readLines(os.Args[1])
	if err != nil {
		fmt.Fprintln(os.Stderr, "readLines:", err)
		os.Exit(1)
	}
	totalgame, totalpower := parseLines(lines)
	fmt.Printf("%d\n%d\n", totalgame, totalpower)
}
