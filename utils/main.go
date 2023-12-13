package utils

import (
	"bufio"
	"cmp"
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

func InRange[T cmp.Ordered](value, lower, upper T) bool {
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

func Map[In any, Out any](input []In, fn func(In) Out) (output []Out) {
	for i := range input {
		output = append(output, fn(input[i]))
	}
	return output
}

func Filter[T any](input []T, fn func(T) bool) (output []T) {
	for i := range input {
		if fn(input[i]) {
			output = append(output, input[i])
		}
	}
	return output
}

func Contains[T comparable](haystack []T, needle T) bool {
	for i := range haystack {
		if haystack[i] == needle {
			return true
		}
	}
	return false
}

func Sum(nums []int) (num int) {
	for i := range nums {
		num += nums[i]
	}
	return num
}

func Number[T int | uint | float32 | float64](num string) T {
	if n, err := strconv.Atoi(num); err == nil {
		return T(n)
	}
	return T(0)
}
