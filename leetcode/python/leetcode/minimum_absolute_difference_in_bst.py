class Solution:
    def _traverse_in_order(self, root):
        if not root:
            return []
        result = self._traverse_in_order(root.left)
        result.append(root.val)
        result += self._traverse_in_order(root.right)
        return result

    def getMinimumDifference(self, root):
        result = float('Inf')
        items = self._traverse_in_order(root)
        for i in range(1, len(items)):
            current_diff = items[i] - items[i-1]
            result = min(result, current_diff)
        return result
