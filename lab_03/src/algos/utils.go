package algos

import "math/rand"

type Stack[T any] struct {
	Data []T
}

func (s *Stack[T]) Push(item T) {
	s.Data = append(s.Data, item)
}

func (s *Stack[T]) Pop() T {
	result := s.Data[len(s.Data)-1]
	s.Data = s.Data[:len(s.Data)-1] // Truncate slice.
	return result
}

func (s *Stack[T]) IsEmpty() bool {
	return len(s.Data) == 0
}

func (s *Stack[T]) Len() int {
	return len(s.Data)
}

func GenerateArray(n int) []int {
	arr := make([]int, n)
	for i := 0; i < n; i++ {
		arr[i] = rand.Intn(n)
	}

	return arr
}
