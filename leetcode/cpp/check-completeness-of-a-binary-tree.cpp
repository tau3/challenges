#include <queue>

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
  bool isCompleteTree(TreeNode *root) {
    bool is_gap = false;
    queue<TreeNode *> dq;
    dq.push(root);
    while (!dq.empty()) {
      TreeNode *node = dq.front();
      dq.pop();

      if (node == nullptr) {
        is_gap = true;
        continue;
      } else {
        if (is_gap) {
          return false;
        } else {
          dq.push(node->left);
          dq.push(node->right);
        }
      }
    }
    return true;
  }
};