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
		//if pivot-left > right-pivot {
		stack.Push([2]int{left, pivot - 1})
		stack.Push([2]int{pivot + 1, right})
		//} else {
		//	stack.Push([2]int{pivot + 1, right})
		//	stack.Push([2]int{left, pivot - 1})
		//}
	}
}

func InsertionSort[T constraints.Ordered](array []T) {
	for i := 1; i < len(array); i++ {
		for j := i - 1; j >= 0 && array[j] > array[j+1]; j-- {
			array[j], array[j+1] = array[j+1], array[j]
		}
	}
}

func BubbleSort[T constraints.Ordered](array []T) {
	for i := 0; i < len(array)-1; i++ {
		for j := 0; j < len(array)-i-1; j++ {
			if array[j] > array[j+1] {
				array[j], array[j+1] = array[j+1], array[j]
			}
		}
	}
}
