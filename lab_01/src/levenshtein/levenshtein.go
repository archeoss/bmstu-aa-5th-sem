package levenshtein

import (
	"math"
)

func DL_STD(a, b string) (int, Matrix) {
	matrix := NewMatrix(len(a), len(b))

	for i := 0; i < len(a)+1; i++ {
		matrix.m[i][0] = i
	}

	for i := 1; i < len(b)+1; i++ {
		matrix.m[0][i] = i
	}

	for i := 1; i < len(a)+1; i++ {
		for j := 1; j < len(b)+1; j++ {
			subCost := 1
			if a[i-1] == b[j-1] {
				subCost = 0
			}
			matrix.m[i][j] = min(matrix.m[i-1][j]+1,
				matrix.m[i][j-1]+1,
				matrix.m[i-1][j-1]+subCost,
			)
			if i > 1 && j > 1 && a[i-1] == b[j-2] && a[i-2] == b[j-1] {
				matrix.m[i][j] = min(matrix.m[i][j], matrix.m[i-2][j-2]+1)
			}
		}
	}

	return matrix.m[len(a)][len(b)], *matrix
}

func findRecSolution(a, b string, i, j int) int {

	if min(i, j) < 0 {
		return math.MaxInt16
	}

	if i == 0 {
		return j
	}

	if j == 0 {
		return i
	}

	subCost := 1
	if a[i-1] == b[j-1] {
		subCost = 0
	}

	if i > 1 && j > 1 && a[i-1] == b[j-2] && a[i-2] == b[j-1] {
		return min(findRecSolution(a, b, i-1, j)+1,
			findRecSolution(a, b, i, j-1)+1,
			findRecSolution(a, b, i-1, j-1)+subCost,
			findRecSolution(a, b, i-2, j-2)+1)
	}

	return min(findRecSolution(a, b, i-1, j)+1,
		findRecSolution(a, b, i, j-1)+1,
		findRecSolution(a, b, i-1, j-1)+subCost,
	)
}

func DL_Recursive(a, b string) int {
	return findRecSolution(a, b, len(a), len(b))
}

func findSolution(a, b string, mat *Matrix, i, j int) int {
	if mat.m[i][j] != math.MaxInt16 {
		return mat.m[i][j]
	}
	m := min(i, j)
	if m < 0 {
		return math.MaxInt16
	}
	if m == 0 {
		mat.m[i][j] = max(i, j)
		return mat.m[i][j]
	}

	subCost := 1
	if a[i-1] == b[j-1] {
		subCost = 0
	}

	if i > 1 && j > 1 && a[i-1] == b[j-2] && a[i-2] == b[j-1] {
		mat.m[i][j] = min(
			findSolution(a, b, mat, i-1, j)+1,
			findSolution(a, b, mat, i, j-1)+1,
			findSolution(a, b, mat, i-1, j-1)+subCost,
			findSolution(a, b, mat, i-2, j-2)+1)
		return mat.m[i][j]
	}

	mat.m[i][j] = min(findSolution(a, b, mat, i-1, j)+1,
		findSolution(a, b, mat, i, j-1)+1,
		findSolution(a, b, mat, i-1, j-1)+subCost,
	)

	return mat.m[i][j]
}

func DL_CacheRecursive(a, b string) (int, Matrix) {
	mat := NewMatrix(len(a), len(b))
	for i := 0; i < len(a)+1; i++ {
		for j := 0; j < len(b)+1; j++ {
			mat.m[i][j] = math.MaxInt16
		}
	}
	findSolution(a, b, mat, len(a), len(b))

	return mat.m[len(a)][len(b)], *mat
}
