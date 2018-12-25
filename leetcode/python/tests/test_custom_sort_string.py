from unittest import TestCase

from leetcode.custom_sort_string import Solution


class CustomSortStringTest(TestCase):
    def test_custom_sort_string(self):
        solution = Solution()
        self.assertEqual('cbad', solution.customSortString('cba', 'abcd'))
