#define CATCH_CONFIG_MAIN

#include <vector>

#include <catch2/catch.hpp>

using std::vector;

int division(int a, int b) {
    int result = 0;
    while (a > 0) {
        a -= b;
        ++result;
    }
    return result;
}

class Solution {
  public:
    vector<int> productExceptSelf(vector<int> &nums) {
        int product = nums[0];
        for (size_t i = 1; i < nums.size(); ++i) {
            product *= nums[i];
        }
        vector<int> result;
        for (int num : nums) {
            int val = division(product, num);
            result.push_back(val);
        }
        return result;
    }
};

TEST_CASE("[1,2,3,4] - [24,12,8,6]", "[P238]") {
    Solution solution;
    const vector<int> expected = {24, 12, 8, 6};
    vector<int> input = {1, 2, 3, 4};
    REQUIRE(expected == solution.productExceptSelf(input));
}

TEST_CASE("[-1,1,0,-3,3] - [0,0,9,0,0]", "[P238]") {
    Solution solution;
    const vector<int> expected = {0, 0, 9, 0, 0};
    vector<int> input = {-1, 1, 0, -3, 3};
    REQUIRE(expected == solution.productExceptSelf(input));
}
