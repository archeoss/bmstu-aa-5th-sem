package main

import (
	"fmt"
	"lab_03/algos"
	_ "lab_03/algos"
	"sort"
)

func check(arr1 []int, arr2 []int) bool {
	for i := 0; i < len(arr1); i++ {
		if arr1[i] != arr2[i] {
			return false
		}
	}
	return true
}

func main() {
	s := 25
	for i := 0; i < 1050; i++ {
		array := algos.GenerateArray(s)
		array2 := make([]int, s)
		s += 25
		copy(array2, array)
		sort.Ints(array)
		algos.InsertionSort(array2)
		if !check(array, array2) {
			fmt.Println("Array incorrect: ", array, array2)
		}
	}
}
