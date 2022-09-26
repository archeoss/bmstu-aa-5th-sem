package levenshtein

import (
	"fmt"
	"testing"
)

const BEGIN = 5
const STEPS = 20
const INC = 5

func BenchmarkDL_CacheRecursive(b *testing.B) {
	for steps, amount := 0, BEGIN; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample_a := GenerateString(amount)
			sample_b := GenerateString(amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				DL_CacheRecursive(sample_a, sample_b)
			}
		})
	}
}

func BenchmarkDL_Recursive(b *testing.B) {
	for steps, amount := 0, BEGIN; steps < 5; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample_a := GenerateString(amount)
			sample_b := GenerateString(amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				DL_Recursive(sample_a, sample_b)
			}
		})
	}
}

func BenchmarkDL_STD(b *testing.B) {
	for steps, amount := 0, BEGIN; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			sample_a := GenerateString(amount)
			sample_b := GenerateString(amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				DL_STD(sample_a, sample_b)
			}
		})
	}
}
