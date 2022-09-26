package levenshtein

import (
	"fmt"
	"golang.org/x/exp/constraints"
	"math/rand"
	"time"
)

func min[T constraints.Ordered](a T, args ...T) T {
	localMin := a
	for _, arg := range args {
		if arg < localMin {
			localMin = arg
		}
	}

	return localMin
}

func max[T constraints.Ordered](a T, args ...T) T {
	localMax := a
	for _, arg := range args {
		if arg > localMax {
			localMax = arg
		}
	}

	return localMax
}

type Matrix struct {
	m [][]int
}

func (matr Matrix) PrintMatrix() {
	for _, nums := range matr.m {
		fmt.Println(nums)
	}
}

func NewMatrix(n, m int) *Matrix {
	res := Matrix{}
	for i := 0; i < n+1; i++ {
		res.m = append(res.m, make([]int, m+1))
	}

	return &res
}

const ALPHABET = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"

func GenerateString(n int) string {
	rand.Seed(time.Now().UnixNano())
	res := ""
	for i := 0; i < n; i++ {
		res += string(ALPHABET[rand.Intn(len(ALPHABET))])
	}

	return res
}

func ReadWord() string {
	var word string
	fmt.Scanln(&word)

	return word
}
