package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("assets/day_02.txt")
	if err != nil {
		panic(err)
	}

	defer file.Close()
	reader := bufio.NewReader(file)

	var result uint32
	for {
		line, err := reader.ReadBytes('\n')
		if err == io.EOF {
			break
		}

		line = bytes.TrimSpace(line)
		parts := strings.Split(string(line), " ")
		switch parts[1] {
		case "X":
			result += 1
			switch parts[0] {
			case "A":
				result += 3
			case "B":
				result += 0
			case "C":
				result += 6
			}
		case "Y":
			result += 2
			switch parts[0] {
			case "A":
				result += 6
			case "B":
				result += 3
			case "C":
				result += 0
			}
		case "Z":
			result += 3
			switch parts[0] {
			case "A":
				result += 0
			case "B":
				result += 6
			case "C":
				result += 3
			}
		}
	}

	fmt.Println(result)
}
