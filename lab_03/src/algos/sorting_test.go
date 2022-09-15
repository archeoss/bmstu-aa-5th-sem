package algos

import (
	"fmt"
	"testing"
)

const STEPS = 15
const INC = 25

func TestQuicksort(t *testing.T) {
	res := [10]int{2, 7, 0, 1, 3, 8, 6, 4, 10, 9}
	Quicksort(res[:])
	correct := [10]int{0, 1, 2, 3, 4, 6, 7, 8, 9, 10}
	if res != correct {
		t.Errorf("Incorrect sort result: got - '%v', correct - '%v'", res[:], correct[:])
	}
}

func TestQuicksortSorted(t *testing.T) {
	res := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	Quicksort(res[:])
	correct := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	if res != correct {
		t.Errorf("Incorrect sort result: got - '%v', correct - '%v'", res[:], correct[:])
	}
}

func TestQuicksortReversed(t *testing.T) {
	res := [10]int{9, 8, 7, 6, 5, 4, 3, 2, 1, 0}
	Quicksort(res[:])
	correct := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	if res != correct {
		t.Errorf("Incorrect sort result: got - '%v', correct - '%v'", res[:], correct[:])
	}
}

func TestInsertionSort(t *testing.T) {
	res := [10]int{2, 7, 0, 1, 3, 8, 6, 4, 10, 9}
	InsertionSort(res[:])
	correct := [10]int{0, 1, 2, 3, 4, 6, 7, 8, 9, 10}
	if res != correct {
		t.Errorf("Incorrect sort result: got - '%v', correct - '%v'", res[:], correct[:])
	}
}

func TestInsertionSortSorted(t *testing.T) {
	res := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	InsertionSort(res[:])
	correct := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	if res != correct {
		t.Errorf("Incorrect sort result: got - '%v', correct - '%v'", res[:], correct[:])
	}
}

func TestInsertionSortReversed(t *testing.T) {
	res := [10]int{9, 8, 7, 6, 5, 4, 3, 2, 1, 0}
	InsertionSort(res[:])
	correct := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	if res != correct {
		t.Errorf("Incorrect sort result: got - '%v', correct - '%v'", res[:], correct[:])
	}
}

func TestBubbleSort(t *testing.T) {
	res := [10]int{2, 7, 0, 1, 3, 8, 6, 4, 10, 9}
	BubbleSort(res[:])
	correct := [10]int{0, 1, 2, 3, 4, 6, 7, 8, 9, 10}
	if res != correct {
		t.Errorf("Incorrect sort result: got - '%v', correct - '%v'", res[:], correct[:])
	}
}

func TestBubbleSortSorted(t *testing.T) {
	res := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	BubbleSort(res[:])
	correct := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	if res != correct {
		t.Errorf("Incorrect sort result: got - '%v', correct - '%v'", res[:], correct[:])
	}
}

func TestBubbleSortReversed(t *testing.T) {
	res := [10]int{9, 8, 7, 6, 5, 4, 3, 2, 1, 0}
	BubbleSort(res[:])
	correct := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	if res != correct {
		t.Errorf("Incorrect sort result: got - '%v', correct - '%v'", res[:], correct[:])
	}
}

func BenchmarkQuicksort(b *testing.B) {
	for steps, amount := 0, 0; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample := GenerateArray(amount)
			sampleCopy := make([]int, amount)
			for i := 0; i < b.N; i++ {
				copy(sampleCopy, sample)
				Quicksort(sampleCopy)
			}

		})
	}
}

func BenchmarkInsertionSort(b *testing.B) {
	for steps, amount := 0, 0; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample := GenerateArray(amount)
			sampleCopy := make([]int, amount)
			for i := 0; i < b.N; i++ {
				copy(sampleCopy, sample)
				InsertionSort(sampleCopy)
			}

		})
	}
}

func BenchmarkBubbleSort(b *testing.B) {
	for steps, amount := 0, 0; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample := GenerateArray(amount)
			sampleCopy := make([]int, amount)
			for i := 0; i < b.N; i++ {
				copy(sampleCopy, sample)
				BubbleSort(sampleCopy)
			}

		})
	}
}