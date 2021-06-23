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

        std::sort(word1.begin(), word1.end());
        std::sort(word2.begin(), word2.end());

        if (word1 == word2) {
            return true;
        }

        return frequencies(word1) == frequencies(word2);
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
