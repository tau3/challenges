class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None


class Solution:
    def deleteDuplicates(self, head: ListNode) -> ListNode:
        if head is None or head.next is None:
            return head

        if head.val == head.next.val:
            current = head.next.next
            while current is not None and current.val == head.val:
                current = current.next
            result = self.deleteDuplicates(current)
        else:
            head.next = self.deleteDuplicates(head.next)
            result = head
        return result
