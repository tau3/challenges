#include <vector>
#include <deque>
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
        deque<TreeNode *> dq;
        dq.push_back(root);
        while (!dq.empty())
        {
            TreeNode *node = dq.front();
            dq.pop_front();

            if (node == nullptr)
            {
                continue;
            }

            dq.push_back(node->left);
            dq.push_back(node->right);

            if ((node->right != nullptr) && (node->left == nullptr))
            {
                return false;
            }
            if ((node->left != nullptr) && (node->right == nullptr))
            {
                if (!has_only_nulls(dq))
                {
                    return false;
                };
            }
        }
        return true;
    }

private:
    bool has_only_nulls(const deque<TreeNode *> &dq)
    {
        for (TreeNode *ptr : dq)
        {
            if (ptr != nullptr)
            {
                return false;
            }
        }
        return true;
    }
};