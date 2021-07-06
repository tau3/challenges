package org.tau.leetcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class MostCommonWordTest {
    private static final MostCommonWord SOLUTION = new MostCommonWord();

    @Test
    void testCase1() {
        String actual = SOLUTION.mostCommonWord("Bob hit a ball, the hit BALL flew far after it was hit.", new String[]{"hit"});
        assertEquals("ball", actual);
    }

    @Test
    void testCase2() {
        String actual = SOLUTION.mostCommonWord("a.", new String[]{});
        assertEquals("a", actual);
    }

    @Test
    void testCase3() {
        String actual = SOLUTION.mostCommonWord("Bob. hIt, baLl", new String[]{"bob", "hit"});
        assertEquals("ball", actual);
    }
}
