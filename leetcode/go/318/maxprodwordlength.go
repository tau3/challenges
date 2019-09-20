package solution

func maxProduct(words []string) int {
	var wordToMask = makeWordToMask(words)
	var max, candidate int
	for i, left := range words {
		for j, right := range words {
			if (i != j) && ((wordToMask[left] & wordToMask[right]) == 0) {
				candidate = len(left) * len(right)
				if max < candidate {
					max = candidate
				}
			}
		}
	}
	return max
}

func makeWordToMask(words []string) map[string]uint {
	var result = make(map[string]uint)
	for _, word := range words {
		var mask uint
		for _, letter := range word {
			mask |= 1 << uint(letter-'a')
		}
		result[word] = mask
	}
	return result
}
