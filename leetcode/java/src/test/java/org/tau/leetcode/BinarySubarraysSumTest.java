package org.tau.leetcode;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class BinarySubarraysSumTest {
    private static final BinarySubarraysSum SOLUTUION = new BinarySubarraysSum();

    @Test
    void testCase1() {
        assertEquals(4, SOLUTUION.numSubarraysWithSum(new int[]{1, 0, 1, 0, 1}, 2));
    }

    @Test
    void testCase2() {
        assertEquals(15, SOLUTUION.numSubarraysWithSum(new int[]{0, 0, 0, 0, 0}, 0));
    }
}
