#include <vector>

using namespace std;

class Solution
{
public:
    bool containsPattern(vector<int> &arr, int m, int k)
    {
        for (int start_slice = 0; start_slice < arr.size() - m; start_slice++)
        {
            int count = 0;
            for (int start_pattern = start_slice + m; start_pattern < arr.size() - m; start_pattern++)
            {
                if (are_slices_equal(arr, start_slice, start_pattern, m))
                {
                    ++count;
                    start_pattern += m;
                }
            }
            if (count == k)
            {
                return true;
            }
        }
        return false;
    }

private:
    bool are_slices_equal(vector<int> &arr, int start_slice, int start_pattern, int m)
    {
        for (int i = 0; i < m; i++)
        {
            if (arr[start_slice + i] != arr[start_pattern + i])
            {
                return false;
            }
        }
        return true;
    }
};