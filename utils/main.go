package utils

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

func Max(i, j int) int {
	if i > j {
		return i
	}
	return j
}

func InRange(value, lower, upper float64) bool {
	return value >= lower && value <= upper
}

func Lines(str string) (lines []string) {
	return strings.Split(strings.ReplaceAll(strings.TrimSpace(str), "\r\n", "\n"), "\n")
}

func Words(str string) []string {
	var i, j int
	words := strings.Split(str, " ")

	// Moving all the empty strings to the end of the array while preserving the
	// order.
	for i = 0; i < len(words); i++ {
		if words[i] = strings.TrimSpace(words[i]); i != j && len(words[i]) != 0 {
			words[i], words[j] = words[j], words[i]
		}
		if len(words[j]) != 0 {
			j++
		}
	}

	return words[:j]
}

func FirstLineOf(file_path string) (string, error) {
	file, err := os.Open(file_path)
	if err != nil {
		return "", err
	}
	defer file.Close()

	file_scanner := bufio.NewScanner(file)
	file_scanner.Scan()

	return file_scanner.Text(), nil
}

func StartsWith(src, dst string) bool {
	if len(dst) > len(src) {
		return false
	}
	return src[:len(dst)] == dst
}

func Float(num string) float64 {
	if n, err := strconv.ParseFloat(num, 64); err == nil {
		return n
	}
	return 0
}

func Integer(num string) int {
	if n, err := strconv.Atoi(num); err == nil {
		return n
	}
	return 0
}
