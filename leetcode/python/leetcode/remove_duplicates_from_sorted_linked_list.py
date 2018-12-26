class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None


class Solution:
    def deleteDuplicates(self, head):
        current = head
        while current and current.next:
            next_ = current.next
            while next_ and (next_.val == current.val):
                next_ = next_.next
            current.next = next_
            current = current.next
        return head
