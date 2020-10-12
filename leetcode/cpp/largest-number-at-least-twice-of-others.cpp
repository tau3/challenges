#include <vector>
#include "gtest/gtest.h"

using namespace std;

class Solution
{
public:
    int dominantIndex(vector<int> &nums)
    {
        int max_value = -1;
        size_t result = -1;
        for (size_t i = 0; i < nums.size(); i++)
        {
            if (nums[i] > max_value)
            {
                result = i;
                max_value = nums[i];
            }
        }

        int required_max = -1;
        for (size_t i = 0; i < nums.size(); i++)
        {
            if (i != result)
            {
                required_max = max(required_max, nums[i] * 2);
            }
        }

        return (max_value >= required_max) ? result : -1;
    }
};

TEST(P747, test_case_1)
{
    Solution solution;
    vector<int> nums{3, 6, 1, 0};
    ASSERT_EQ(1, solution.dominantIndex(nums));
}

TEST(P747, test_case_2)
{
    Solution solution;
    vector<int> nums{1, 2, 3, 4};
    ASSERT_EQ(-1, solution.dominantIndex(nums));
}