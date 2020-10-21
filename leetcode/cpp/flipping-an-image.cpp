#include <algorithm>
#include <vector>

using namespace std;

class Solution {
  public:
    vector<vector<int>> flipAndInvertImage(vector<vector<int>> &A) {
        for (vector<int> &row : A) {
            reverse(row.begin(), row.end());
            inverse(row);
        }
        return A;
    }

  private:
    void inverse(vector<int> &vec) const {
        for (int &i : vec) {
            i = !i;
        }
    }
};