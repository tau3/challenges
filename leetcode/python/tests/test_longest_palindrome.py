from unittest import TestCase

from leetcode.longest_palindrome import Solution


class LongestPalindromeTest(TestCase):
    def test_longest_palindrome(self):
        solution = Solution()
        self.assertEqual(7, solution.longestPalindrome('abccccdd'))
