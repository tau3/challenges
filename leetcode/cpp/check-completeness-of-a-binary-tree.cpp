#include <vector>
#include <deque>
#include <iostream>

using namespace std;

struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution
{
public:
    bool isCompleteTree(TreeNode *root)
    {
        bool is_gap = false;
        deque<TreeNode *> dq{root};
        while (!dq.empty())
        {
            TreeNode *node = dq.front();
            dq.pop_front();

            if (node == nullptr)
            {
                is_gap = true;
                continue;
            }
            else
            {
                if (is_gap)
                {
                    return false;
                }
                else
                {
                    dq.push_back(node->left);
                    dq.push_back(node->right);
                }
            }
        }
        return true;
    }
};