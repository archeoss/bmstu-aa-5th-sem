package main

import (
	"fmt"
	"github.com/logrusorgru/aurora"
	"lab_03/levenshtein"
)

func main() {

	fmt.Println(aurora.Magenta("Расстояние Левенштейна\n"))

	fmt.Printf("Введите первое слово: ")
	a := levenshtein.ReadWord()
	fmt.Printf("Введите второе слово: ")
	b := levenshtein.ReadWord()

	fmt.Println()

	distRec := levenshtein.DL_Recursive(a, b)
	fmt.Println(aurora.Yellow("Рекурсивный метод Дамерау-Левенштейна без заполнения матрицы:"))
	fmt.Printf("Расстояние: %d\n\n", aurora.Green(distRec))

	fmt.Println()

	distRM, matRM := levenshtein.DL_CacheRecursive(a, b)
	fmt.Println(aurora.Yellow("Рекурсивный метод с заполнением матрицы:"))
	matRM.PrintMatrix()
	fmt.Printf("Расстояние: %d\n\n", aurora.Green(distRM))

	fmt.Println()

	distDL, matDL := levenshtein.DL_STD(a, b)
	fmt.Println(aurora.Yellow("Метод Дамерау - Левенштейна:"))
	matDL.PrintMatrix()
	fmt.Printf("Расстояние: %d\n\n", aurora.Green(distDL))
}
