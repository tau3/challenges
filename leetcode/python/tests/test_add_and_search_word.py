from unittest import TestCase

from leetcode.add_and_search_word import WordDictionary


class AddAndSearchWordTest(TestCase):
    def test_solution(self):
        solution = WordDictionary()
        solution.addWord("bad")
        solution.addWord("dad")
        solution.addWord("mad")
        self.assertFalse(solution.search("pad"))
        self.assertTrue(solution.search("bad"))
        self.assertTrue(solution.search(".ad"))
        self.assertTrue(solution.search("b.."))
