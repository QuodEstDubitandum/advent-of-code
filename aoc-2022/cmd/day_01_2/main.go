package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"os"
	"slices"
	"strconv"
)

func main() {
	file, err := os.Open("assets/day_01.txt")
	if err != nil {
		panic(err)
	}

	defer file.Close()
	reader := bufio.NewReader(file)

	var sums []uint64
	var currSum uint64
	for {
		line, err := reader.ReadBytes('\n')
		if err == io.EOF {
			fmt.Println("Found EOF")
			break
		}

		line = bytes.TrimSpace(line)
		if len(line) == 0 {
			sums = append(sums, currSum)
			currSum = 0
			continue
		}

		num, err := strconv.ParseUint(string(line), 10, 64)
		currSum += num
	}

	slices.SortFunc(sums, sortDesc)
	result := sums[0] + sums[1] + sums[2]
	fmt.Println(result)
}

func sortDesc(a uint64, b uint64) int {
	if a > b {
		return -1
	}
	return 1
}
