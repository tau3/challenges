class Solution:
    def customSortString(self, S, T):
        index = {}
        for i, letter in enumerate(S):
            index[letter] = i
        counter = [''] * 27
        for letter in T:
            pos = index[letter] if letter in index else 26
            counter[pos] += letter
        return ''.join(counter)
