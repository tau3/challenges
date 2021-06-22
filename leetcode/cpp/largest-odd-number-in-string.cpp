#define CATCH_CONFIG_MAIN

#include <catch2/catch.hpp>
#include <string>

using namespace std;

int char_to_int(char c) {
  switch (c) {
  case '0':
    return 0;
  case '1':
    return 1;
  case '2':
    return 2;
  case '3':
    return 3;
  case '4':
    return 4;
  case '5':
    return 5;
  case '6':
    return 6;
  case '7':
    return 7;
  case '8':
    return 8;
  case '9':
    return 9;
  }
  throw "unreachable";
}

class Solution {
public:
  string largestOddNumber(string num) {
    for (int i = num.size() - 1; i > -1; --i) {
      int val = char_to_int(num[i]);
      if (val % 2 != 0) {
        return num.substr(0, i + 1);
      }
    }
    return "";
  }
};

TEST_CASE("5 -> 52", "[P1903]") {
  Solution solution;
  REQUIRE("5" == solution.largestOddNumber("52"));
}

TEST_CASE("empty -> 4206", "[P1903]") {
  Solution solution;
  REQUIRE("" == solution.largestOddNumber("4206"));
}

TEST_CASE("35427 -> 35427", "[P1903]") {
  Solution solution;
  REQUIRE("35427" == solution.largestOddNumber("35427"));
}

TEST_CASE("dummy", "[P1903]") { REQUIRE(5 == 5); }
