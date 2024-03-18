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

func StartsWith(src, dst string) bool {
	if len(dst) > len(src) {
		return false
	}
	return src[:len(dst)] == dst
}

func Map[In any, Out any](input []In, fn func(In) Out) (output []Out) {
	for i := range input {
		output = append(output, fn(input[i]))
	}
	return output
}

func Filter[T any](input []T, fn func(T) bool) []T {
	var i int
	for j := range input {
		if fn(input[j]) {
			input[i], i = input[j], i+1
		}
	}
	return input[:i]
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

func Float(num string) float64 {
	if n, err := strconv.ParseFloat(num, 64); err == nil {
		return n
	}
	return 0
}

func Integer[T int | uint](num string) T {
	if n, err := strconv.Atoi(num); err == nil {
		return T(n)
	}
	return T(0)
}
