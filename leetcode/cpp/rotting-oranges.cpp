#include <deque>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
  public:
    int orangesRotting(vector<vector<int>> &grid) {
        deque<pair<int, int>> rotten;
        int fresh_count = 0;
        for (int r = 0; r < grid.size(); ++r) {
            vector<int> row = grid[r];
            for (int c = 0; c < row.size(); ++c) {
                if (grid[r][c] == 2) {
                    rotten.push_back(make_pair(r, c));
                } else if (grid[r][c] == 1) {
                    ++fresh_count;
                }
            }
        }
        if (fresh_count == 0) {
            return 0;
        }

        int ticks = 0;
        while (!rotten.empty()) {
            ++ticks;
            auto orange = rotten.front();
            rotten.pop_front();
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
        if ((point.second - 1) > 0) {
            auto left = make_pair(point.first, point.second - 1);
            result.push_back(left);
        }
        if ((point.second + 1) < width) {
            auto right = make_pair(point.first, point.second + 1);
            result.push_back(right);
        }
        if ((point.first - 1) > 0) {
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