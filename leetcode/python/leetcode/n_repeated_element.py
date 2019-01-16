class Solution:
    def repeatedNTimes(self, A):
        items = set()
        for e in A:
            if e in items:
                return e
            items.add(e)
        raise Exception('no solution')
