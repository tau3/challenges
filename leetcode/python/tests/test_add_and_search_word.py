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

    def test_failed_case(self):
        solution = WordDictionary()
        solution.addWord("at")
        solution.addWord("and")
        solution.addWord("an")
        solution.addWord("add")
        self.assertFalse(solution.search("a"))
        self.assertFalse(solution.search(".at"))
        solution.addWord("bat")
        self.assertTrue(solution.search(".at"))
        self.assertTrue(solution.search("an."))
        self.assertFalse(solution.search("a.d."))
        self.assertFalse(solution.search("b."))
        self.assertTrue(solution.search("a.d"))
        self.assertFalse(solution.search("."))
