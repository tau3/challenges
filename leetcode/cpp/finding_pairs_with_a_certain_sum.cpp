#define CATCH_CONFIG_MAIN

#include <unordered_map>
#include <vector>

#include <catch2/catch.hpp>

using std::vector;

class FindSumPairs {
    vector<int> &nums1;
    vector<int> &nums2;
    std::unordered_map<int, int> nums2_to_freq;

  public:
    FindSumPairs(vector<int> &nums1, vector<int> &nums2)
        : nums1(nums1), nums2(nums2) {
        for (int num2 : nums2) {
            nums2_to_freq[num2]++;
        }
    }

    void add(int index, int val) {
        int old_val = nums2[index];
        nums2_to_freq[old_val]--;
        if (nums2_to_freq[old_val] < 0) {
            nums2_to_freq.erase(old_val);
        }
        int new_val = old_val + val;
        nums2[index] = new_val;
        nums2_to_freq[new_val]++;
    }

    int count(int tot) {
        int result = 0;
        for (int num1 : nums1) {
            result += nums2_to_freq[tot - num1];
        }
        return result;
    }
};

TEST_CASE("example 1", "[P1865]") {
    vector<int> nums1 = {1, 1, 2, 2, 2, 3};
    vector<int> nums2 = {1, 4, 5, 2, 5, 4};
    FindSumPairs findSumPairs = FindSumPairs(nums1, nums2);
    REQUIRE(8 == findSumPairs.count(7));
    findSumPairs.add(3, 2);
    REQUIRE(2 == findSumPairs.count(8));
    REQUIRE(1 == findSumPairs.count(4));
    findSumPairs.add(0, 1);
    findSumPairs.add(1, 1);
    REQUIRE(11 == findSumPairs.count(7));
}
