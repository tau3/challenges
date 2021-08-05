#define CATCH_CONFIG_MAIN

#include <algorithm>
#include <vector>

#include <catch2/catch.hpp>

using std::vector;

class Solution {
  public:
    vector<int> minSubsequence(vector<int> &nums) {
        std::sort(nums.rbegin(), nums.rend());

        int sum = 0;
        for (int num : nums) {
            sum += num;
        }
        const int half = sum / 2;

        int current = 0;
        auto it = nums.begin();
        while (it != nums.end()) {
            current += *it;
            if (current > half) {
                break;
            }
            ++it;
        }
        ++it;
        nums.erase(it, nums.end());
        return nums;
    }
};

TEST_CASE("[4,3,10,9,8] - [10,9]", "[P1403]") {
    Solution solution;
    vector<int> input = {4, 3, 10, 9, 8};
    vector<int> actual = solution.minSubsequence(input);
    REQUIRE(vector<int>({10, 9}) == actual);
}

TEST_CASE("[4,4,7,6,7] - [7,7,6]", "[P1403]") {
    Solution solution;
    vector<int> input = {4, 4, 7, 6, 7};
    vector<int> actual = solution.minSubsequence(input);
    REQUIRE(vector<int>({7, 7, 6}) == actual);
}

TEST_CASE("[6] - [6]", "[P1403]") {
    Solution solution;
    vector<int> input = {6};
    vector<int> actual = solution.minSubsequence(input);
    REQUIRE(vector<int>({6}) == actual);
}
