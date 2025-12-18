#include "catch2/catch_test_macros.hpp"
#include <algorithm>
#include <cstddef>
#include <iostream>
#include <optional>
#include <utility>
#include <vector>

using namespace std;

using Result = pair<string, optional<tuple<string, size_t, size_t>>>;

Result almost_sorted(vector<int> &arr) {
  auto copy = arr;
  sort(copy.begin(), copy.end());

  if (copy == arr) {
    return {"yes", {}};
  }

  vector<size_t> unsorted;
  for (size_t i = 0; i < arr.size(); i++) {
    if (arr[i] != copy[i]) {
      unsorted.push_back(i);
    }
  }

  if (unsorted.size() == 1) {
    return {"no", {}};
  }

  if (unsorted.size() == 2) {
    swap(arr[unsorted[0]], arr[unsorted[1]]);
    if (arr == copy) {
      return {"yes", {{"swap", unsorted[0] + 1, unsorted[1] + 1}}};
    } else {
      return {"no", {}};
    }
  }

  for (size_t i = 1; i < unsorted.size(); i++) {
    if (unsorted[i] != (unsorted[i - 1] + 1)) {
      return {"no", {}};
    }
    if (arr[unsorted[i]] >= arr[unsorted[i - 1]]) {
      return {"no", {}};
    }
  }
  return {"yes",
          {{"reverse", unsorted[0] + 1, unsorted[unsorted.size() - 1] + 1}}};
}

void integration(vector<int> arr) {
  const auto &[first, second] = almost_sorted(arr);
  cout << first << endl;
  if (second != nullopt) {
    const auto &[str, l, r] = second.value();
    cout << str << " " << l << " " << r << endl;
  }
}

TEST_CASE("sample 1", "almost_sorted") {
  const Result expected = {"yes", {{"swap", 1, 2}}};
  vector<int> input = {4, 2};
  const Result actual = almost_sorted(input);
  REQUIRE(expected == actual);
}

TEST_CASE("sample 2", "almost_sorted") {
  const Result expected = {"no", {}};
  vector<int> input = {3, 1, 2};
  const Result actual = almost_sorted(input);
  REQUIRE(expected == actual);
}

TEST_CASE("sample 3", "almost_sorted") {
  const Result expected = {"yes", {{"reverse", 2, 5}}};
  vector<int> input = {1, 5, 4, 3, 2, 6};
  const Result actual = almost_sorted(input);
  REQUIRE(expected == actual);
}
