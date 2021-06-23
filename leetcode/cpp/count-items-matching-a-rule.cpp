#define CATCH_CONFIG_MAIN

#include <string>
#include <vector>

#include <catch2/catch.hpp>

using namespace std;

class Solution {
  public:
    int countMatches(vector<vector<string>> &items, string ruleKey,
                     string ruleValue) {
        int result = 0;
        for (vector<string> tuple : items) {
            size_t index = -1;
            if (ruleKey == "type") {
                index = 0;
            } else if (ruleKey == "color") {
                index = 1;
            } else if (ruleKey == "name") {
                index = 2;
            }

            const string actual_value = tuple[index];
            if (actual_value == ruleValue) {
                ++result;
            }
        }
        return result;
    }
};

TEST_CASE("case 1", "[P1773]") {
    Solution solution;
    vector<vector<string>> items = {{"phone", "blue", "pixel"},
                                    {"computer", "silver", "lenovo"},
                                    {"phone", "gold", "iphone"}};
    REQUIRE(1 == solution.countMatches(items, "color", "silver"));
}

TEST_CASE("case 2", "[P1773]") {
    Solution solution;
    vector<vector<string>> items = {{"phone", "blue", "pixel"},
                                    {"computer", "silver", "phone"},
                                    {"phone", "gold", "iphone"}};
    REQUIRE(2 == solution.countMatches(items, "type", "phone"));
}
