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
        deque<TreeNode *> q;
        q.push_back(root);
        while (!q.empty())
        {
            TreeNode *node = q.front();
            q.pop_front();

            if (node == nullptr)
            {
                continue;
            }

            q.push_back(node->left);
            q.push_back(node->right);

            if ((node->right != nullptr) && (node->left == nullptr))
            {
                return false;
            }
            if ((node->left != nullptr) && (node->right == nullptr))
            {
                if (!has_only_nulls(q))
                {
                    return false;
                };
            }
        }
        return true;
    }

private:
    bool has_only_nulls(const deque<TreeNode *> &q)
    {
        for (TreeNode *ptr : q)
        {
            if (ptr != nullptr)
            {
                return false;
            }
        }
        return true;
    }
};