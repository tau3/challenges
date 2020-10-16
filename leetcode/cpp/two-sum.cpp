#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
  public:
    vector<int> twoSum(vector<int> &nums, int target) {
        unordered_map<int, int> cache;
        for (int i = 0; i < nums.size(); ++i) {
            int value = target - nums[i];
            if (cache.find(value) != cache.end()) {
                return vector<int>{i, cache[value]};
            } else {
                cache[nums[i]] = i;
            }
        }
        throw "no solution";
    }
};