#include <vector>
#include "gtest/gtest.h"
#include <iostream>

using namespace std;

class Solution
{
public:
    Solution() : pattern_length(0), repeats(0), arr(0) {}

    bool containsPattern(vector<int> &arr, int m, int k)
    {
        this->arr = arr;
        pattern_length = m;
        repeats = k;

        for (int start_slice = 0; start_slice < int(arr.size()) - pattern_length * (repeats - 1); start_slice++)
        {
            int count = 1;
            for (int start_pattern = start_slice + pattern_length;
                 start_pattern <= int(arr.size()) - pattern_length;)
            {
                if (are_slices_equal(start_slice, start_pattern))
                {
                    ++count;
                    start_pattern += pattern_length;
                }
                else
                {
                    break;
                }
            }
            if (count >= repeats)
            {
                return true;
            }
        }
        return false;
    }

private:
    int pattern_length;
    int repeats;
    vector<int> arr;

    bool are_slices_equal(int start_slice, int start_pattern)
    {
        for (int i = 0; i < pattern_length; i++)
        {
            if (arr[start_slice + i] != arr[start_pattern + i])
            {
                return false;
            }
        }
        return true;
    }
};

TEST(TestSuite, test_case_1)
{
    Solution solution;
    vector<int> input{1, 2, 4, 4, 4, 4};
    ASSERT_TRUE(solution.containsPattern(input, 1, 3));
}

TEST(TestSuite, test_case_2)
{
    Solution solution;
    vector<int> input{1, 2, 1, 2, 1, 1, 1, 3};
    ASSERT_TRUE(solution.containsPattern(input, 2, 2));
}

TEST(TestSuite, test_case_3)
{
    Solution solution;
    vector<int> input{1, 2, 1, 2, 1, 3};
    ASSERT_FALSE(solution.containsPattern(input, 2, 3));
}

TEST(TestSuite, test_case_4)
{
    Solution solution;
    vector<int> input{1, 2, 3, 1, 2};
    ASSERT_FALSE(solution.containsPattern(input, 2, 2));
}

TEST(TestSuite, test_case_5)
{
    Solution solution;
    vector<int> input{2, 2, 2, 2};
    ASSERT_FALSE(solution.containsPattern(input, 2, 3));
}

TEST(TestSuite, test_case_6)
{
    Solution solution;
    vector<int> input{2, 2};
    ASSERT_TRUE(solution.containsPattern(input, 1, 2));
}