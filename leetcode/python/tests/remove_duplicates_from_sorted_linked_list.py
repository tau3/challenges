from unittest import TestCase

from leetcode.remove_duplicates_from_sorted_linked_list import Solution, ListNode


class RemoveDuplicatesFromSortedLinkedListTest(TestCase):
    def test_remove(self):
        self.assertEqual([1, 2, 3], self._get_actual([1, 1, 2, 3, 3]))
        self.assertEqual([1, 2], self._get_actual([1, 1, 2]))
        self.assertEqual([1], self._get_actual([1]))
        self.assertEqual([1], self._get_actual([1, 1]))
        self.assertEqual([1], self._get_actual([1, 1, 1]))
        self.assertEqual([], self._get_actual([]))

    def _get_actual(self, values):
        solution = Solution()
        head = self._list_to_nodes(values)
        actual = solution.deleteDuplicates(head)
        return self._nodes_to_list(actual)

    @staticmethod
    def _list_to_nodes(values):
        result = ListNode(-1)
        prev = result
        for value in values:
            node = ListNode(value)
            prev.next = node
            prev = node
        return result.next

    @staticmethod
    def _nodes_to_list(head):
        result = []
        current = head
        while current:
            result.append(current.val)
            current = current.next
        return result
