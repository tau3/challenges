_END = '*'


class WordDictionary:
    def __init__(self):
        self.__root = _Trie()

    def addWord(self, word: str) -> None:
        self.__root.add_word(word + _END)

    def search(self, word: str) -> bool:
        return self.__root.contains(word + _END)


class _Trie:
    def __init__(self):
        self.__children = {}

    def add_word(self, word):
        current = self.__children
        for letter in word:
            if letter not in current:
                current[letter] = {}
            current = current[letter]

    def contains(self, word):
        return self.__dfs(word, self.__children)

    def __dfs(self, word, node):
        letter = word[0]
        rest = word[1:]

        if letter == '.':
            for child in node.values():
                if self.__dfs(rest, child):
                    return True
            return False
        if letter == _END:
            return _END in node
        if letter not in node:
            return False
        return self.__dfs(rest, node[letter])
