package org.tau.leetcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

class MagicDictionaryTest {
    @Test
    void testCase1() {
        MagicDictionary dictionary = new MagicDictionary();
        dictionary.buildDict(new String[]{"hello", "leetcode"});
        assertFalse(dictionary.search("hello"));
        assertTrue(dictionary.search("hhllo"));
        assertFalse(dictionary.search("hell"));
        assertFalse(dictionary.search("leetcoded"));
    }
}
