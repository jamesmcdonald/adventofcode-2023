package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
	"unicode"
)

func readInput(filename string) []string {
	file, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	lines := []string{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines
}

type Schematic struct {
	schematic   string
	pos         int
	width       int
	starmap     map[int]int
	gear_ratios []int
}

func NewSchematic(schematic string, width int) *Schematic {
	return &Schematic{
		schematic:   schematic,
		pos:         width + 1,
		width:       width,
		starmap:     make(map[int]int),
		gear_ratios: []int{},
	}
}

func (s *Schematic) GetAdjacentRow(pos int, up bool, len int) []rune {
	adjacent := []rune{}
	if up {
		adjacent = []rune(s.schematic)[pos-(s.width+2)-1 : pos-(s.width+2)+len+2]
	} else {
		adjacent = []rune(s.schematic)[pos+(s.width+2)-1 : pos+(s.width+2)+len+2]
	}
	return adjacent
}

func (s *Schematic) IsPart(len int) bool {
	adjacent := []rune{}
	adjacent = append(adjacent, s.GetAdjacentRow(s.pos, true, len)...)
	c := []rune(s.schematic)[s.pos-1]
	adjacent = append(adjacent, c)
	c = []rune(s.schematic)[s.pos+len]
	adjacent = append(adjacent, c)
	adjacent = append(adjacent, s.GetAdjacentRow(s.pos, false, len)...)
	count := 0
	for _, c := range adjacent {
		if c != '.' {
			count += 1
		}
	}
	return count > 0
}

func (s *Schematic) GearRatios(part int, len int) {
	nearby := []int{}
	for i := s.pos - (s.width + 2) - 1; i < s.pos-(s.width+2)+len+1; i++ {
		nearby = append(nearby, i)
	}
	for i := s.pos + (s.width + 2) - 1; i < s.pos+(s.width+2)+len+1; i++ {
		nearby = append(nearby, i)
	}
	nearby = append(nearby, s.pos-1)
	nearby = append(nearby, s.pos+len)
	for _, loc := range nearby {
		c := []rune(s.schematic)[loc]
		if c == '*' {
			val, ok := s.starmap[loc]
			if ok {
				s.gear_ratios = append(s.gear_ratios, part*val)
				total := 0
				for _, v := range s.gear_ratios {
					total += v
				}
				fmt.Println(total)
			} else {
				s.starmap[loc] = part
			}
		}
	}
}

func (s *Schematic) Next() int {
	part := 0
	length := 0
	for s.pos < len(s.schematic) {
		c := []rune(s.schematic)[s.pos]
		if !unicode.IsDigit(c) {
			s.pos += 1
			continue
		}
		for unicode.IsDigit(c) {
			part = part*10 + int(c-'0')
			length += 1
			c = []rune(s.schematic)[s.pos+length]
		}
		if s.IsPart(length) {
			s.GearRatios(part, length)
			s.pos += length
			return part
		}
		s.pos += length
		part = 0
		length = 0
	}
	return 0
}

func main() {
	lines := readInput(os.Args[1])
	width := len(lines[0])
	for i, line := range lines {
		lines[i] = "." + line + "."
	}
	end := strings.Repeat(".", width+2)
	lines = append([]string{end}, lines...)
	lines = append(lines, end)
	total := 0
	schematic := NewSchematic(strings.Join(lines, ""), width)
	for {
		part := schematic.Next()
		if part == 0 {
			break
		}
		total += part
	}
	fmt.Println(total)
}
