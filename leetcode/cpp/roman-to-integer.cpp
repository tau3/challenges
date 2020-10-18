#include "gtest/gtest.h"
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
  public:
    int romanToInt(string s) {
        int result = 0;
        size_t length = s.length();
        for (size_t i = 0; i < length; ++i) {
            int value = roman_to_value().at(s[i]);
            size_t next = i + 1;
            if (next < length) {
                int next_value = roman_to_value().at(s[next]);
                if (next_value > value) {
                    value *= -1;
                }
            }
            result += value;
        }
        return result;
    }

  private:
    const unordered_map<char, int> &roman_to_value() {
        static unordered_map<char, int> roman_to_value = {
            {'I', 1},   {'V', 5},   {'X', 10},  {'L', 50},
            {'C', 100}, {'D', 500}, {'M', 1000}};
        return roman_to_value;
    }
};

TEST(online_election_2, test_case_2) {
    Solution solution;
    ASSERT_EQ(3, solution.romanToInt("III"));
    ASSERT_EQ(4, solution.romanToInt("IV"));
    ASSERT_EQ(9, solution.romanToInt("IX"));
    ASSERT_EQ(58, solution.romanToInt("LVIII"));
    ASSERT_EQ(1994, solution.romanToInt("MCMXCIV"));
}