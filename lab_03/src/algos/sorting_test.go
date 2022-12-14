package algos

import (
	"fmt"
	"testing"
)

const BEGIN = 25
const STEPS = 15
const INC = 100

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

func TestCountingSort(t *testing.T) {
	res := [10]int{2, 7, 0, 1, 3, 8, 6, 4, 10, 9}
	CountingSort(res[:])
	correct := [10]int{0, 1, 2, 3, 4, 6, 7, 8, 9, 10}
	if res != correct {
		t.Errorf("Incorrect sort result: got - '%v', correct - '%v'", res[:], correct[:])
	}
}

func TestCountingSortSorted(t *testing.T) {
	res := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	CountingSort(res[:])
	correct := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	if res != correct {
		t.Errorf("Incorrect sort result: got - '%v', correct - '%v'", res[:], correct[:])
	}
}

func TestCountingSortReversed(t *testing.T) {
	res := [10]int{9, 8, 7, 6, 5, 4, 3, 2, 1, 0}
	CountingSort(res[:])
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

func BenchmarkQuicksortRandom(b *testing.B) {
	for steps, amount := 0, 0; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample := GenerateArray(amount)
			sampleCopy := make([]int, amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				//b.StopTimer()		// Disabled Due performance reasons
				copy(sampleCopy, sample)
				//b.StartTimer()	// Disabled Due performance reasons
				Quicksort(sampleCopy)
			}
		})
	}
}

func BenchmarkCountingSortRandom(b *testing.B) {
	for steps, amount := 0, 0; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample := GenerateArray(amount)
			sampleCopy := make([]int, amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				//b.StopTimer()		// Disabled Due performance reasons
				copy(sampleCopy, sample)
				//b.StartTimer()	// Disabled Due performance reasons
				CountingSort(sampleCopy)
			}
		})
	}
}

func BenchmarkBubbleSortRandom(b *testing.B) {
	for steps, amount := 0, 0; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample := GenerateArray(amount)
			sampleCopy := make([]int, amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				//b.StopTimer()		// Disabled Due performance reasons
				copy(sampleCopy, sample)
				//b.StartTimer()	// Disabled Due performance reasons
				BubbleSort(sampleCopy)
			}
		})
	}
}

func BenchmarkQuicksortSorted(b *testing.B) {
	for steps, amount := 0, 0; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample := GenerateSortedArray(amount)
			sampleCopy := make([]int, amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				//b.StopTimer()		// Disabled Due performance reasons
				copy(sampleCopy, sample)
				//b.StartTimer()	// Disabled Due performance reasons
				Quicksort(sampleCopy)
			}
		})
	}
}

func BenchmarkCountingSortSorted(b *testing.B) {
	for steps, amount := 0, 0; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample := GenerateSortedArray(amount)
			sampleCopy := make([]int, amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				//b.StopTimer()		// Disabled Due performance reasons
				copy(sampleCopy, sample)
				//b.StartTimer()	// Disabled Due performance reasons
				CountingSort(sampleCopy)
			}
		})
	}
}

func BenchmarkBubbleSortSorted(b *testing.B) {
	for steps, amount := 0, 0; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample := GenerateSortedArray(amount)
			sampleCopy := make([]int, amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				//b.StopTimer()		// Disabled Due performance reasons
				copy(sampleCopy, sample)
				//b.StartTimer()	// Disabled Due performance reasons
				BubbleSort(sampleCopy)
			}
		})
	}
}

func BenchmarkQuicksortReversed(b *testing.B) {
	for steps, amount := 0, 0; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample := GenerateReversedArray(amount)
			sampleCopy := make([]int, amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				//b.StopTimer()		// Disabled Due performance reasons
				copy(sampleCopy, sample)
				//b.StartTimer()	// Disabled Due performance reasons
				Quicksort(sampleCopy)
			}
		})
	}
}

func BenchmarkCountingSortReversed(b *testing.B) {
	for steps, amount := 0, 0; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample := GenerateReversedArray(amount)
			sampleCopy := make([]int, amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				//b.StopTimer()		// Disabled Due performance reasons
				copy(sampleCopy, sample)
				//b.StartTimer()	// Disabled Due performance reasons
				CountingSort(sampleCopy)
			}
		})
	}
}

func BenchmarkBubbleSortReversed(b *testing.B) {
	for steps, amount := 0, 0; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample := GenerateReversedArray(amount)
			sampleCopy := make([]int, amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				//b.StopTimer()		// Disabled Due performance reasons
				copy(sampleCopy, sample)
				//b.StartTimer()	// Disabled Due performance reasons
				BubbleSort(sampleCopy)
			}
		})
	}
}
