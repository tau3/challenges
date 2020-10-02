#include <vector>

using namespace std;

class Solution
{
public:
    bool containsPattern(vector<int> &arr, int m, int k)
    {
        this->arr = arr;
        pattern_length = m;
        repeats = k;

        for (int start_slice = 0; start_slice < arr.size() - pattern_length; ++start_slice)
        {
            int count = 0;
            for (int start_pattern = start_slice + pattern_length; start_pattern < arr.size() - pattern_length; start_pattern++)
            {
                if (are_slices_equal(start_slice, start_pattern))
                {
                    ++count;
                    start_pattern += pattern_length;
                }
            }
            if (count == repeats)
            {
                return true;
            }
        }
        return false;
    }

private:
    int pattern_length;
    int repeats;
    vector<int> &arr;

    bool are_slices_equal(int start_slice, int start_pattern)
    {
        for (int i = 0; i < pattern_length; i++)
        {
            if (arr[start_slice + i] != arr[start_pattern + i])
            {
                return false;
            }
        }
        return true;
    }
};