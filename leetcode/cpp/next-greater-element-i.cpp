#include <vector>
#include "gtest/gtest.h"
#include <algorithm>

using namespace std;

class Solution
{
public:
    vector<int> nextGreaterElement(vector<int> &nums1, vector<int> &nums2)
    {
        vector<int> result(nums1.size(), -1);
        for (int i = 0; i < static_cast<int>(nums1.size()); i++)
        {
            for (int j = index_of(nums2, nums1[i]) + 1; j < static_cast<int>(nums2.size()); j++)
            {
                if (nums2[j] > nums1[i])
                {
                    result[i] = nums2[j];
                    break;
                }
            }
        }
        return result;
    }

private:
    int index_of(const vector<int> &nums, int element)
    {
        auto iterator = find(nums.begin(), nums.end(), element);
        return iterator - nums.begin();
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

TEST(P496, test_case_2)
{
    vector<int> nums1{2, 4};
    vector<int> nums2{1, 2, 3, 4};
    vector<int> expected{3, -1};
    Solution solution;
    ASSERT_EQ(expected, solution.nextGreaterElement(nums1, nums2));
}