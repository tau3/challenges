package org.tau.leetcode;

import java.util.Deque;
import java.util.LinkedList;

public class PalindromeLinkedList {
    public boolean isPalindrome(ListNode head) {
        ListNode current = head;
        Deque<Integer> values = new LinkedList<>();
        while (current != null) {
            values.add(current.val);
            current = current.next;
        }

        boolean result = true;
        while (values.size() > 1) {
            int top = values.removeFirst();
            int bottom = values.removeLast();
            if (top != bottom) {
                result = false;
                break;
            }
        }
        return result;
    }

    private static class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }
}
