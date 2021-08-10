#define CATCH_CONFIG_MAIN

#include <algorithm>
#include <vector>

#include <catch2/catch.hpp>

using std::vector;

class Solution {
  public:
    int distanceBetweenBusStops(vector<int> &distance, int start,
                                int destination) {
        if (start > destination) {
            start += destination;
            destination = start - destination;
            start -= destination;
        }

        int forward = 0;
        for (int i = start; i < destination; ++i) {
            forward += distance[i];
        }

        int backward = 0;
        int i = start - 1;
        if (i < 0) {
            i = distance.size() - 1;
        }
        while (i != destination - 1) {
            backward += distance[i];
            --i;
            if (i < 0) {
                i = distance.size() - 1;
            }
        }

        return std::min(forward, backward);
    }
};

TEST_CASE("distance = [1,2,3,4], start = 0, destination = 1", "[P1184]") {
    Solution solution;
    vector<int> distance = {1, 2, 3, 4};
    REQUIRE(1 == solution.distanceBetweenBusStops(distance, 0, 1));
}

TEST_CASE("distance = [1,2,3,4], start = 0, destination = 2", "[P1184]") {
    Solution solution;
    vector<int> distance = {1, 2, 3, 4};
    REQUIRE(3 == solution.distanceBetweenBusStops(distance, 0, 2));
}

TEST_CASE("distance = [1,2,3,4], start = 0, destination = 3", "[P1184]") {
    Solution solution;
    vector<int> distance = {1, 2, 3, 4};
    REQUIRE(4 == solution.distanceBetweenBusStops(distance, 0, 3));
}
