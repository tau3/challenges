#include <vector>
#include "gtest/gtest.h"

using namespace std;

class Solution
{
public:
    vector<int> selfDividingNumbers(int left, int right)
    {
        vector<int> result;
        for (int i = left; i <= right; ++i)
        {
            if (is_self_dividing(i))
            {
                result.push_back(i);
            }
        }
        return result;
    }

private:
    bool is_self_dividing(int num)
    {
        vector<int> digits = to_digits(num);
        if (find(digits.begin(), digits.end(), 0) != digits.end())
        {
            return false;
        }
        for (int i : digits)
        {
            if ((num % i) != 0)
            {
                return false;
            }
        }
        return true;
    }

    vector<int> to_digits(int num)
    {
        vector<int> result;
        while (num != 0)
        {
            int rem = num % 10;
            result.push_back(rem);
            num /= 10;
        }
        return result;
    }
};

TEST(P728, test_case_1)
{
    Solution solution;
    vector<int> expected{1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22};
    ASSERT_EQ(expected, solution.selfDividingNumbers(1, 22));
}