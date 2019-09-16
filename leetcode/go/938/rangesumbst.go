package solution

// TreeNode kekeke
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rangeSumBST(root *TreeNode, L int, R int) int {
	if root == nil {
		return 0
	}

	var result int
	if (root.Val >= L) && (root.Val <= R) {
		result = root.Val
	} else {
		result = 0
	}

	result += rangeSumBST(root.Left, L, R)
	result += rangeSumBST(root.Right, L, R)

	return result
}
