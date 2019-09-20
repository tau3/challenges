package soluiton

var mapping = [...]string{".-", "-...", "-.-.", "-..", ".", "..-.", "--.",
	"....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.",
	"...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."}

func toMorse(word string) string {
	var index int
	var result string
	for _, letter := range word {
		index = toIndex(letter)
		result += mapping[index]
	}
	return result
}

func toIndex(char rune) int {
	return int(char) - 97
}

func countDistincts(words []string) int {
	var cache = make(map[string]bool)
	for _, word := range words {
		cache[word] = true
	}
	return len(cache)
}

func uniqueMorseRepresentations(words []string) int {
	var morse string
	var morseWords = make([]string, len(words))
	for i, word := range words {
		morse = toMorse(word)
		morseWords[i] = morse
	}
	return countDistincts(morseWords)
}
