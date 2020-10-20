#include <algorithm>
#include <stdlib.h>

using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right)
        : val(x), left(left), right(right) {}
};

class Solution {
  public:
    int maxAncestorDiff(TreeNode *root) {
        if (!root || (!root->left && !root->right)) {
            return -1;
        }

        TreeNode *most_left = get_most_left(root);
        TreeNode *most_right = get_most_right(root);

        int diff_to_left = !most_left ? 0 : abs(most_left->val - root->val);
        int diff_to_right = !most_right ? 0 : abs(most_right->val - root->val);
        int max_diff = max(diff_to_left, diff_to_right);

        int max_diff_ancestors =
            max(maxAncestorDiff(root->left), maxAncestorDiff(root->right));

        return max(max_diff, max_diff_ancestors);
    }

  private:
    TreeNode *get_most_left(TreeNode *root) {
        TreeNode *result = root;
        while (result) {
            result = result->left;
        }
        return result;
    }

    TreeNode *get_most_right(TreeNode *root) {
        TreeNode *result = root;
        while (result) {
            result = result->right;
        }
        return result;
    }
};