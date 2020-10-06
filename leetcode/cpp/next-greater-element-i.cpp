#include <vector>
#include "gtest/gtest.h"

using namespace std;

class Solution
{
public:
    vector<int> nextGreaterElement(vector<int> &nums1, vector<int> &nums2)
    {
        vector<int> result(nums1.size(), -1);
        for (int i = 0; i < static_cast<int>(nums1.size()); i++)
        {
            int lvalue = nums1[i];
            int next = lvalue;
            for (int rvalue : nums2)
            {
                if (rvalue > lvalue)
                {
                    next = (next > lvalue) ? min(next, rvalue) : rvalue;
                }
            }
            result[i] = (nums1[i] != lvalue) ? lvalue : -1;
        }
        return result;
    }
};

TEST(P496, test_case_1)
{
    vector<int> nums1{4, 1, 2};
    vector<int> nums2{1, 3, 4, 2};
    vector<int> expected{-1, 3, -1};
    Solution solution;
    ASSERT_EQ(expected, solution.nextGreaterElement(nums1, nums2));
}