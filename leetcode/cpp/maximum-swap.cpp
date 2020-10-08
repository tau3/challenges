#include <string>
#include "gtest/gtest.h"

using namespace std;

class Solution
{
public:
    int maximumSwap(int num)
    {
        string number = to_string(num);

        vector<int> last(10, -1);
        for (size_t i = 0; i < number.size(); i++)
        {
            last[number[i] - '0'] = i;
        }

        for (int i = 0; i < static_cast<int>(number.size()); i++)
        {
            for (int j = 9; j > i; j--)
            {
                if (last[j] > i)
                {
                    swap(number[i], number[last[j]]);
                    return stoi(number);
                }
            }
        }
        return num;
    }
};

TEST(P670, test_case_1)
{
    Solution solution;
    ASSERT_EQ(7236, solution.maximumSwap(2736));
    ASSERT_EQ(9973, solution.maximumSwap(9973));
    ASSERT_EQ(3, solution.maximumSwap(3));
    ASSERT_EQ(21, solution.maximumSwap(12));
}