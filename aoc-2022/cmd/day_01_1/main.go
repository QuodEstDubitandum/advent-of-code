package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("assets/day_01.txt")
	if err != nil {
		panic(err)
	}

	defer file.Close()
	reader := bufio.NewReader(file)

	var result uint64
	var currSum uint64
	for {
		line, err := reader.ReadBytes('\n')
		if err == io.EOF {
			fmt.Println("Found EOF")
			break
		}

		line = bytes.TrimSpace(line)
		if len(line) == 0 {
			result = max(result, currSum)
			currSum = 0
			continue
		}

		num, err := strconv.ParseUint(string(line), 10, 64)
		currSum += num
	}

	fmt.Println(result)
}
