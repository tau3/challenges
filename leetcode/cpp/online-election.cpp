#include <vector>
#include "gtest/gtest.h"

using namespace std;

class TopVotedCandidate
{
public:
    TopVotedCandidate(vector<int> &persons, vector<int> &times)
    {
        map<int, int> person_to_votes;
        int most_votes = 0;
        int elected = -1;
        for (size_t i = 0; i < persons.size(); ++i)
        {
            person_to_votes[persons[i]]++;
            int votes = person_to_votes[persons[i]];
            if (votes >= most_votes)
            {
                most_votes = votes;
                elected = persons[i];
            }
            time_to_solution[times[i]] = elected;
        }
    }

    int q(int t)
    {
        return (*prev((time_to_solution.upper_bound(t)))).second;
    }

private:
    map<int, int> time_to_solution;
};

TEST(online_election_1, test_case_1)
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

TEST(online_election_2, test_case_2)
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