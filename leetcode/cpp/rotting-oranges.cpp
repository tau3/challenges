#include "gtest/gtest.h"
#include <deque>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
  public:
    int orangesRotting(vector<vector<int>> &grid) {
        deque<pair<int, int>> rotten;
        int fresh_count = 0;
        pair<int, int> marker = make_pair(-1, -1);
        for (size_t r = 0; r < grid.size(); ++r) {
            vector<int> row = grid[r];
            for (size_t c = 0; c < row.size(); ++c) {
                if (grid[r][c] == 2) {
                    rotten.push_back(make_pair(r, c));
                } else if (grid[r][c] == 1) {
                    ++fresh_count;
                }
            }
        }
        rotten.push_back(marker);
        if (fresh_count == 0) {
            return 0;
        }

        int ticks = 0;
        while (!rotten.empty()) {
            auto orange = rotten.front();
            rotten.pop_front();
            if (orange == marker) {
                if (rotten.empty()) {
                    break;
                }
                ++ticks;
                rotten.push_back(marker);
                continue;
            }
            for (pair<int, int> adjacent : adjacents(orange, grid)) {
                if (grid[adjacent.first][adjacent.second] == 1) {
                    grid[adjacent.first][adjacent.second] = 2;
                    rotten.push_back(adjacent);
                    --fresh_count;
                }
            }
        }

        return (fresh_count == 0) ? ticks : -1;
    }

  private:
    vector<pair<int, int>> adjacents(pair<int, int> point,
                                     const vector<vector<int>> &grid) {
        int height = grid.size();
        int width = grid[0].size();

        vector<pair<int, int>> result;
        if (point.second > 0) {
            auto left = make_pair(point.first, point.second - 1);
            result.push_back(left);
        }
        if ((point.second + 1) < width) {
            auto right = make_pair(point.first, point.second + 1);
            result.push_back(right);
        }
        if (point.first > 0) {
            auto top = make_pair(point.first - 1, point.second);
            result.push_back(top);
        }
        if ((point.first + 1) < height) {
            auto bottom = make_pair(point.first + 1, point.second);
            result.push_back(bottom);
        }
        return result;
    }
};

TEST(p994, test_case_1) {
    Solution solution;
    vector<vector<int>> grid = {{2, 1, 1}, {1, 1, 0}, {0, 1, 1}};
    ASSERT_EQ(4, solution.orangesRotting(grid));
}

TEST(p994, test_case_2) {
    Solution solution;
    vector<vector<int>> grid = {{2, 1, 1}, {0, 1, 1}, {1, 0, 1}};
    ASSERT_EQ(-1, solution.orangesRotting(grid));
}

TEST(p994, test_case_3) {
    Solution solution;
    vector<vector<int>> grid = {{0, 2}};
    ASSERT_EQ(0, solution.orangesRotting(grid));
}

TEST(p994, test_case_4) {
    Solution solution;
    vector<vector<int>> grid = {{1, 2}};
    ASSERT_EQ(1, solution.orangesRotting(grid));
}