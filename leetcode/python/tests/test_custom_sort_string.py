from unittest import TestCase

from leetcode.custom_sort_string import Solution


class LongestPalindromeTest(TestCase):
    def test_longest_palindrome(self):
        solution = Solution()
        self.assertEqual('cbad', solution.customSortString('cba', 'abcd'))
