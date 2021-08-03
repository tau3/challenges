#define CATCH_CONFIG_MAIN

#include <vector>

#include <catch2/catch.hpp>

using std::vector;

class Solution {
  public:
    vector<int> productExceptSelf(vector<int> &nums) {
        const size_t size = nums.size();
        vector<int> result(size, 1);
        for (size_t i = 1; i < size; ++i) {
            result[i] = result[i - 1] * nums[i - 1];
        }

        int post_product = 1;
        for (int i = size - 2; i >= 0; --i) {
            post_product *= nums[i + 1];
            result[i] *= post_product;
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
