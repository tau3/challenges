class WordDictionary:
    def __init__(self):
        self.__root = _Node('.')

    def addWord(self, word: str) -> None:
        self.__root.add_word(word + '*')

    def search(self, word: str) -> bool:
        return self.__root.contains(word + '*')


class _Node:
    def __init__(self, val):
        self.__val = val
        self.__children = {}

    def add_word(self, word):
        if not word:
            return

        letter = word[0]
        rest = word[1:]

        child = self.__put_in_absent(letter)
        child.add_word(rest)
        child = self.__put_in_absent('.')
        child.add_word(rest)

    def __put_in_absent(self, letter):
        if letter not in self.__children:
            self.__children[letter] = _Node(letter)
        return self.__children[letter]

    def contains(self, word):
        if not word:
            return True
        letter = word[0]
        rest = word[1:]

        result = False
        if letter in self.__children:
            subtree = self.__children[letter]
            result = subtree.contains(rest)
        return result
