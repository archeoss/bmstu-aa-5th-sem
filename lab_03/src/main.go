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
	//fmt.Println(aurora.Magenta("Расстояние Левенштейна\n"))
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
	//
	//fmt.Printf("Введите первое слово: ")
	//fWord := levenshtein.ReadWord()
	//fmt.Printf("Введите второе слово: ")
	//sWord := levenshtein.ReadWord()
	//
	//fmt.Println()
	//
	//distRec := levenshtein.Recursive(fWord, sWord)
	//fmt.Println(aurora.Yellow("Рекурсивный метод без заполнения матрицы:"))
	//fmt.Printf("Расстояние: %d\n\n", aurora.Green(distRec))
	//
	//fmt.Println()
	//
	//distRM, matRM := levenshtein.RecursiveMatrix(fWord, sWord)
	//fmt.Println(aurora.Yellow("Рекурсивный метод с заполнением матрицы:"))
	//matRM.PrintMatrix()
	//fmt.Printf("Расстояние: %d\n\n", aurora.Green(distRM))
	//
	//fmt.Println()
	//
	//distIt, matIt := levenshtein.IterativeMatrix(fWord, sWord)
	//fmt.Println(aurora.Yellow("Итеративный метод с заполнением матрицы:"))
	//matIt.PrintMatrix()
	//fmt.Printf("Расстояние: %d\n\n", aurora.Green(distIt))
	//
	//fmt.Println()
	//
	//distDL, matDL := levenshtein.DamerauLevenshtein(fWord, sWord)
	//fmt.Println(aurora.Yellow("Метод Дамерау - Левенштейна:"))
	//matDL.PrintMatrix()
	//fmt.Printf("Расстояние: %d\n\n", aurora.Green(distDL))
}
