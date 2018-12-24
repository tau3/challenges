from collections import Counter


class Solution:
    def longestPalindrome(self, s):
        counter = Counter(s)
        pairs = sum(map(lambda v: v // 2, counter.values()))
        result = pairs * 2
        if len(s) > result:
            result += 1
        return result
