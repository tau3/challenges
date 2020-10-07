#include <vector>
#include "gtest/gtest.h"

using namespace std;

class TopVotedCandidate
{
public:
    TopVotedCandidate(vector<int> &persons, vector<int> &times)
    {
        this->persons = persons;
        this->times = times;
    }

    int q(int t)
    {
        vector<int> count(persons.size(), 0);
        for (int i = 0; i < static_cast<int>(persons.size()); i++)
        {
            if (times[i] > t)
            {
                break;
            }
            count[persons[i]]++;
        }

        int biggest = *max_element(count.begin(), count.end());

        vector<int> candidates;
        for (int i = 0; i < static_cast<int>(count.size()); i++)
        {
            if (count[i] == biggest)
            {
                candidates.push_back(i);
            }
        }
        if (candidates.size() == 1)
        {
            return candidates[0];
        }

        int last = -1;
        for (int i = 0; i < static_cast<int>(persons.size()); i++)
        {
            if (times[i] > t)
            {
                break;
            }
            if (find(candidates.begin(), candidates.end(), persons[i]) != candidates.end())
            {
                last = persons[i];
            }
        }

        return last;
    }

private:
    vector<int> persons;
    vector<int> times;
};

TEST(online_election, test_case_1)
{
    vector<int> persons{0, 1, 1, 0, 0, 1, 0};
    vector<int> times{0, 5, 10, 15, 20, 25, 30};
    TopVotedCandidate top_voted_cadidate{persons, times};

    ASSERT_EQ(0, top_voted_cadidate.q(3));
    ASSERT_EQ(1, top_voted_cadidate.q(12));
    ASSERT_EQ(1, top_voted_cadidate.q(25));
    ASSERT_EQ(0, top_voted_cadidate.q(15));
    ASSERT_EQ(0, top_voted_cadidate.q(24));
    ASSERT_EQ(1, top_voted_cadidate.q(8));
}

TEST(online_election, test_case_2)
{
    vector<int> persons{0, 0, 1, 1, 2};
    vector<int> times{0, 67, 69, 74, 87};
    TopVotedCandidate top_voted_cadidate{persons, times};

    ASSERT_EQ(0, top_voted_cadidate.q(4));
    ASSERT_EQ(0, top_voted_cadidate.q(62));
    ASSERT_EQ(1, top_voted_cadidate.q(100));
    ASSERT_EQ(1, top_voted_cadidate.q(88));
    ASSERT_EQ(0, top_voted_cadidate.q(70));
    ASSERT_EQ(0, top_voted_cadidate.q(73));
    ASSERT_EQ(0, top_voted_cadidate.q(22));
    ASSERT_EQ(1, top_voted_cadidate.q(75));
    ASSERT_EQ(0, top_voted_cadidate.q(29));
    ASSERT_EQ(0, top_voted_cadidate.q(10));
}