class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None


class Solution:
    def deleteDuplicates(self, head: ListNode) -> ListNode:
        if not head:
            return None
        if head.next is None:
            return head

        current = head.next
        prev = head
        val = current.val
        while current:
            if val == current.val:
                prev.next = current.next
            else:
                val = current.val
            prev = current
            current = current.next
        return head
