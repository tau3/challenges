from unittest import TestCase

from leetcode.sort_colors import Solution


class SortColorsTest(TestCase):
    def test_sort_colors(self):
        solution = Solution()
        nums = [2, 0, 2, 1, 1, 0]
        solution.sortColors(nums)
        self.assertEqual([0, 0, 1, 1, 2, 2], nums)
