package algos

import "golang.org/x/exp/constraints"

func partition[T constraints.Ordered](arr []T, left int, right int) int {
	pivot := arr[right]
	i := left - 1
	for j := left; j <= right-1; j++ {
		if arr[j] <= pivot {
			i++
			arr[i], arr[j] = arr[j], arr[i]
		}
	}
	arr[i+1], arr[right] = arr[right], arr[i+1]

	return i + 1
}

func Quicksort[T constraints.Ordered](array []T) {
	if len(array) < 2 {
		return
	}

	left := 0
	right := len(array) - 1
	stack := Stack[[2]int]{}
	stack.Push([2]int{left, right})
	for !stack.IsEmpty() {
		lr := stack.Pop()
		left, right = lr[0], lr[1]
		if right <= left {
			continue
		}
		pivot := partition(array, left, right)
		stack.Push([2]int{left, pivot - 1})
		stack.Push([2]int{pivot + 1, right})
	}
}

func CountingSort(array []int) {
	maxVal := array[0]
	for _, val := range array {
		if val > maxVal {
			maxVal = val
		}
	}
	temp := make([]int, maxVal+1)
	for _, val := range array {
		temp[val]++
	}

	j, i := 0, 0

	for i < len(temp) && j < len(array) {
		if temp[i] > 0 {
			array[j] = i
			j++
			temp[i]--
		} else {
			i++
		}
	}
}

func BubbleSort[T constraints.Ordered](array []T) {
	n := len(array) - 1
	for i := 0; i < n; i++ {
		for j := 0; j < n-i; j++ {
			if array[j] > array[j+1] {
				array[j], array[j+1] = array[j+1], array[j]
			}
		}
	}
}
