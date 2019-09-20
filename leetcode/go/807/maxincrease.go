package solution

func maxIncreaseKeepingSkyline(grid [][]int) int {
	var maxX, maxY, res, height int
	for rowNum, row := range grid {
		for colNum, val := range row {
			maxX = maxInRow(grid, rowNum)
			maxY = maxInCol(grid, colNum)
			height = min(maxX, maxY)
			if height > val {
				res += height - val
			}
		}
	}
	return res
}

func min(a int, b int) int {
	if a > b {
		return b
	}
	return a
}

func maximum(array []int) int {
	var m int
	for i, e := range array {
		if i == 0 || e > m {
			m = e
		}
	}
	return m
}

func maxInRow(grid [][]int, rowNum int) int {
	return maximum(grid[rowNum])
}

func maxInCol(grid [][]int, colNum int) int {
	column := make([]int, len(grid))
	for i, row := range grid {
		column[i] = row[colNum]
	}
	return maximum(column)
}
