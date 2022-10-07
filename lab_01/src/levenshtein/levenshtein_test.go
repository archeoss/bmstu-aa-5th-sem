package levenshtein

import (
	"flag"
	"fmt"
	"runtime"
	"testing"
)

const BEGIN = 5
const STEPS = 20
const INC = 5

// Get Size of Stack
var ch = make(chan byte)
var n = flag.Int("n", 3*1e6, "Number of goroutines to create")

func TestDL_CacheRecursive(t *testing.T) {
	flag.Parse()
	sample_a := GenerateString(120)
	sample_b := GenerateString(120)

	var m0 runtime.MemStats
	runtime.ReadMemStats(&m0)

	//go func() {
	for i := 0; i < *n; i++ {
		DL_CacheRecursive(sample_a, sample_b)
	}
	//	<-ch
	//}()
	//runtime.Gosched()
	runtime.GC()

	var m1 runtime.MemStats
	runtime.ReadMemStats(&m1)

	fmt.Printf("  Memory: %.2f bytes\n", float64(m1.Sys-m0.Sys)/float64(*n))
	fmt.Println(m1.Mallocs - m0.Mallocs)
}

// Get Size of Stack
func TestDL_STD(t *testing.T) {

	flag.Parse()
	sample_a := GenerateString(120)
	sample_b := GenerateString(120)

	sample_a := "привет"
	sample_b := "мудило"

	var m0 runtime.MemStats
	runtime.ReadMemStats(&m0)

	//go func() {
	for i := 0; i < *n; i++ {
		DL_STD(sample_a, sample_b)
	}
	//	<-ch
	//}()
	//runtime.Gosched()
	runtime.GC()

	var m1 runtime.MemStats
	runtime.ReadMemStats(&m1)

	fmt.Printf("  Memory: %.2f bytes\n", float64(m1.Sys-m0.Sys)/float64(*n))
	//fmt.Println(m1.Mallocs - m0.Mallocs)
}

func BenchmarkDL_CacheRecursive(b *testing.B) {
	for steps, amount := 0, BEGIN; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			//sample_a := GenerateString(amount)
			//sample_b := GenerateString(amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				sample_a := GenerateString(amount)
				sample_b := GenerateString(amount)

				DL_CacheRecursive(sample_a, sample_b)
			}
		})
	}
}

//func BenchmarkDL_Recursive(b *testing.B) {
//	for steps, amount := 0, BEGIN; steps < 2; steps++ {
//		amount += INC
//		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
//			sample_a := GenerateString(amount)
//			sample_b := GenerateString(amount)
//			b.ResetTimer()
//			for i := 0; i < b.N; i++ {
//				DL_Recursive(sample_a, sample_b)
//			}
//		})
//	}
//}

func BenchmarkDL_STD(b *testing.B) {
	for steps, amount := 0, BEGIN; steps < STEPS; steps++ {
		amount += INC
		b.Run(fmt.Sprintf("size=%d", amount), func(b *testing.B) {
			//sample_a := GenerateString(amount)
			//sample_b := GenerateString(amount)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				sample_a := GenerateString(amount)
				sample_b := GenerateString(amount)

				DL_STD(sample_a, sample_b)
			}
		})
	}
}
