package solution

import "strings"

var vowels = "aeiouAEIOU"

func reverseVowels(s string) string {
	var res = []rune(s)
	var i, j = 0, len(s) - 1
	for i < j {
		if isVowel(s[i]) {
			if isVowel(s[j]) {
				swap(res, i, j)
				i++
			}
			j--
		} else {
			i++
		}
	}
	return string(res)
}

func isVowel(b byte) bool {
	return strings.IndexByte(vowels, b) != -1
}

func swap(runes []rune, i int, j int) {
	var temp = runes[i]
	runes[i] = runes[j]
	runes[j] = temp
}
