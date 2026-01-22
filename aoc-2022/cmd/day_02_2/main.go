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
			switch parts[0] {
			case "A":
				result += 3
			case "B":
				result += 1
			case "C":
				result += 2
			}
		case "Y":
			result += 3
			switch parts[0] {
			case "A":
				result += 1
			case "B":
				result += 2
			case "C":
				result += 3
			}
		case "Z":
			result += 6
			switch parts[0] {
			case "A":
				result += 2
			case "B":
				result += 3
			case "C":
				result += 1
			}
		}
	}

	fmt.Println(result)
}
