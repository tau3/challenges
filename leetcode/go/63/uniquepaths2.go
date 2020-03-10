package solution

func uniquePathsWithObstacles(obstacleGrid [][]int) int {
	if obstacleGrid[0][0] == 1 {
		return 0
	}

	var memo = make([][]int, len(obstacleGrid))
	for i := range memo {
		memo[i] = make([]int, len(obstacleGrid[0]))
	}
	memo[0][0] = 1

	for r, row := range obstacleGrid {
		for c := range row {
			if obstacleGrid[r][c] == 0 {
				if c > 0 {
					memo[r][c] += memo[r][c-1]
				}
				if r > 0 {
					memo[r][c] += memo[r-1][c]
				}
			}
		}
	}
	return memo[len(obstacleGrid)-1][len(obstacleGrid[0])-1]
}
