package org.tau.leetcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class MostCommonWordTest {
    private static final MostCommonWord SOLUTION = new MostCommonWord();

    @Test
    void testCase1() {
        assertEquals("ball", SOLUTION.mostCommonWord("Bob hit a ball, the hit BALL flew far after it was hit.", new String[]{"hit"}));
    }
}
