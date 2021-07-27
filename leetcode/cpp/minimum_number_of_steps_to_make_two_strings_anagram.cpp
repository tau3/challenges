#define CATCH_CONFIG_MAIN

#include <string>
#include <unordered_map>

#include <catch2/catch.hpp>

using std::string;

class Solution {
  public:
    int minSteps(string s, string t) {
        std::unordered_map<char, size_t> s_counts;
        for (char letter : s) {
            s_counts[letter]++;
        }

        std::unordered_map<char, size_t> t_counts;
        for (char letter : t) {
            t_counts[letter]++;
        }

        int result = 0;
        for (auto entry : s_counts) {
            const char letter = entry.first;
            const size_t s_count = entry.second;
            const size_t t_count = t_counts[letter];

            if (s_count > t_count) {
                result += s_count - t_count;
            }
        }
        return result;
    }
};

TEST_CASE("bab - aba - 1", "p1347") {
    Solution solution;
    REQUIRE(1 == solution.minSteps("bab", "aba"));
}

TEST_CASE("leetcode - practice - 5", "p1347") {
    Solution solution;
    REQUIRE(5 == solution.minSteps("leetcode", "practice"));
}

TEST_CASE("anagram - mangaar - 0", "p1347") {
    Solution solution;
    REQUIRE(0 == solution.minSteps("anagram", "mangaar"));
}

TEST_CASE("xxyyzz - xxyyzz - 0", "p1347") {
    Solution solution;
    REQUIRE(0 == solution.minSteps("xxyyzz", "xxyyzz"));
}

TEST_CASE("frient - family - 4", "p1347") {
    Solution solution;
    REQUIRE(4 == solution.minSteps("friend", "family"));
}
