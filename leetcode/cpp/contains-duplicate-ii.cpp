#include "gtest/gtest.h"
#include <map>
#include <vector>

using namespace std;

class Solution {
public:
  bool containsNearbyDuplicate(vector<int> &nums, int k) {
    map<int, int> cache;
    for (int i = 0; i < static_cast<int>(nums.size()); ++i) {
      int num = nums[i];
      auto pos = cache.find(num);
      if (cache.find(num) == cache.end()) {
        cache[num] = i;
      } else {
        if ((i - pos->second) <= k) {
          return true;
        }
        cache[num] = i;
      }
    }
    return false;
  }
};

TEST(P213, test_case_1) {
  Solution solution;
  vector<int> nums = {1, 2, 3, 1};
  ASSERT_TRUE(solution.containsNearbyDuplicate(nums, 3));
}

TEST(P213, test_case_2) {
  Solution solution;
  vector<int> nums = {1, 0, 1, 1};
  ASSERT_TRUE(solution.containsNearbyDuplicate(nums, 1));
}

TEST(P213, test_case_3) {
  Solution solution;
  vector<int> nums = {1, 2, 3, 1, 2, 3};
  ASSERT_FALSE(solution.containsNearbyDuplicate(nums, 2));
}
