class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def pathSum(self, root, target):
        if not root:
            return 0
        return self._calculate_paths(root, target) + \
            self.pathSum(root.left, target) + self.pathSum(root.right, target)

    def _calculate_paths(self, node, target):
        if not node:
            return 0

        result = 1 if node.val == target else 0
        new_target = target - node.val
        result += self._calculate_paths(node.left, new_target) + \
            self._calculate_paths(node.right, new_target)
        return result
