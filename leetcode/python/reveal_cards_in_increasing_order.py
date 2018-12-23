def reveal(deck):
    result = []
    is_reveal = True
    while(deck):
        card = deck.pop(0)
        if is_reveal:
            result.append(card)
        else:
            if deck:
                deck.append(card)
            else:
                result.append(card)
        is_reveal = not is_reveal
    return result


class Solution(object):
    def deckRevealedIncreasing(self, deck):
        
        pass

def main():
    solution = Solution()
    expected = [2, 13, 3, 11, 5, 17, 7]
    actual = solution.deckRevealedIncreasing([17, 13, 11, 2, 3, 5, 7])
    print(expected == actual)

    
if __name__ == '__main__':
    main()
