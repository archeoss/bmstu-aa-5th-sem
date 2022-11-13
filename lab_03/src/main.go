package main

import (
	"fmt"
	"lab_03/algos"
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
	// Enter size of array
	var n int
	fmt.Println("Enter size of array")
	fmt.Scan(&n)

	arr := make([]int, n)
	// Enter array
	fmt.Println("Enter array")
	for i := 0; i < n; i++ {
		fmt.Scan(&arr[i])
	}
	arr1 := make([]int, n)
	copy(arr1, arr)
	arr2 := make([]int, n)
	copy(arr2, arr)
	arr3 := make([]int, n)
	copy(arr3, arr)

	sort.Ints(arr)
	// Sort the array using the algorithm
	algos.BubbleSort(arr1)
	algos.CountingSort(arr2)
	algos.Quicksort(arr3)

	fmt.Println("Sorted array: ", arr)
	fmt.Println()
	// Check the result
	fmt.Println("Result of BubbleSort:")
	fmt.Println(arr1)
	if check(arr, arr1) {
		fmt.Println("OK")
	} else {
		fmt.Println("FAIL")
	}
	fmt.Println("Result of CountingSort:")
	fmt.Println(arr2)
	if check(arr, arr2) {
		fmt.Println("OK")
	} else {
		fmt.Println("FAIL")
	}
	fmt.Println("Result of Quicksort:")
	fmt.Println(arr3)
	if check(arr, arr3) {
		fmt.Println("OK")
	} else {
		fmt.Println("FAIL")
	}
}
