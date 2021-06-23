#define CATCH_CONFIG_MAIN

#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

#include <catch2/catch.hpp>

using namespace std;

vector<int> frequencies(const string &word) {
    vector<int> counts(26);
    for (size_t i = 0; i < word.size(); ++i) {
        const char letter = word[i];
        counts[letter - 97]++;
    }

    vector<int> result;
    for (int val : counts) {
        if (val != 0) {
            result.push_back(val);
        }
    }
    std::sort(result.begin(), result.end());
    return result;
}

class Solution {
  public:
    bool closeStrings(string word1, string word2) {
        if (word1.size() != word2.size()) {
            return false;
        }

        vector<int> frequencies1(26);
        for (size_t i = 0; i < word1.size(); ++i) {
            const char letter = word1[i];
            frequencies1[letter - 'a']++;
        }

        vector<int> frequencies2(26);
        for (size_t i = 0; i < word2.size(); ++i) {
            const char letter = word2[i];
            if (frequencies1[letter - 'a'] == 0) {
                return false;
            }
            frequencies2[letter - 'a']++;
        }

        std::sort(frequencies1.begin(), frequencies1.end());
        std::sort(frequencies2.begin(), frequencies2.end());
        return frequencies1 == frequencies2;
    }
};

TEST_CASE("abc - bca - true", "[P1657]") {
    Solution solution;
    REQUIRE(solution.closeStrings("abc", "bca"));
}

TEST_CASE("a - aa - false", "[P1657]") {
    Solution solution;
    REQUIRE_FALSE(solution.closeStrings("a", "aa"));
}

TEST_CASE("cabbba - abbccc - true", "[P1657]") {
    Solution solution;
    REQUIRE(solution.closeStrings("cabbba", "abbccc"));
}

TEST_CASE("cabbba - aabbss - false", "[P1657]") {
    Solution solution;
    REQUIRE_FALSE(solution.closeStrings("cabbba", "aabbss"));
}

TEST_CASE("uau - ssx - false", "[P1657]") {
    Solution solution;
    REQUIRE_FALSE(solution.closeStrings("uau", "ssx"));
}
