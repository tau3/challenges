from unittest import TestCase

from leetcode.remove_duplicates_from_sorted_list_2 import Solution, ListNode


class RemoveDuplicatesFromSortedList2Test(TestCase):
    def test_delete_duplicates(self):
        node = ListNode(1)
        solution = Solution()
        result = solution.deleteDuplicates(node)
        self.assertEqual(1, result.val)
        self.assertIsNone(result.next)

    def test_delete_when_none(self):
        solution = Solution()
        result = solution.deleteDuplicates(None)
        self.assertIsNone(result)
