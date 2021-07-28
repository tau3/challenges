#include <vector>

using std::vector;

class FindSumPairs {
    vector<int> &nums1;
    vector<int> &nums2;

  public:
    FindSumPairs(vector<int> &nums1, vector<int> &nums2)
        : nums1(nums1), nums2(nums2) {}

    void add(int index, int val) { nums2[index] += val; }

    int count(int tot) {
        int result = 0;
        for (int num1 : nums1) {
            for (int num2 : nums2) {
                if ((num1 + num2) == tot) {
                    ++result;
                }
            }
        }
        return result;
    }
};
