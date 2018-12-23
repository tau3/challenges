from collections import Counter


class Solution:
    def findTheDifference(self, s, t):
        s = Counter(s)
        t = Counter(t)
        result = t - s
        result = next(iter(result))
        return result


if __name__ == '__main__':
    solution = Solution()
    print(solution.findTheDifference("abcd", "abcde"))
    print(solution.findTheDifference("a", "aa"))
