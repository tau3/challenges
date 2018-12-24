class Solution:
    def numJewelsInStones(self, J, S):
        jewels = set(J)
        result = filter(lambda stone: stone in jewels, S)
        return len(list(result))


if __name__ == '__main__':
    solution = Solution()
    print(solution.numJewelsInStones('aA', 'aAAbbbb'))
    print(solution.numJewelsInStones('z', 'ZZ'))
