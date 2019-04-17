class Solution:
    def sortArrayByParity(self, A):
        evens = []
        odds = []
        for a in A:
            if a % 2 == 0:
                odds.append(a)
            else:
                evens.append(a)
        return odds + evens
