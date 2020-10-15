#include "gtest/gtest.h"
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
public:
  int distributeCandies(vector<int> &candies) {
    unordered_set<int> types(candies.begin(), candies.end());
    return min(types.size(), candies.size() / 2);
  }
};

TEST(P575, test_case_1) {
  Solution solution;
  vector<int> candies = {1, 1, 2, 2, 3, 3};
  ASSERT_EQ(3, solution.distributeCandies(candies));
}

TEST(P575, test_case_2) {
  Solution solution;
  vector<int> candies = {1, 1, 2, 3};
  ASSERT_EQ(2, solution.distributeCandies(candies));
}

TEST(P575, test_case_3) {
  Solution solution;
  vector<int> candies = {1, 1};
  ASSERT_EQ(1, solution.distributeCandies(candies));
}

TEST(P575, test_case_4) {
  Solution solution;
  vector<int> candies = {1, 11};
  ASSERT_EQ(1, solution.distributeCandies(candies));
}

TEST(P575, test_case_5) {
  Solution solution;
  vector<int> candies = {2, 2};
  ASSERT_EQ(1, solution.distributeCandies(candies));
}