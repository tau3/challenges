#include <algorithm>
#include <iostream>
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
        int result = -1;
        traverse(root, INT_MIN, INT_MAX, result);
        return result;
    }

  private:
    void traverse(TreeNode *current, int big, int small, int &result) {
        if (!current) {
            return;
        }
        big = max(big, current->val);
        small = min(small, current->val);
        result = max(result, abs(big - small));
        traverse(current->left, big, small, result);
        traverse(current->right, big, small, result);
    }
};