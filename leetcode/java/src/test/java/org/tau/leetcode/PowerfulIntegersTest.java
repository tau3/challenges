package org.tau.leetcode;

import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertEquals;

class PowerfulIntegersTest {
    private static final PowerfulIntegers SOLUTION = new PowerfulIntegers();

    @Test
    void testCase1() {
        assertEquals(Set.of(2, 3, 4, 5, 7, 9, 10), new HashSet<>(SOLUTION.powerfulIntegers(2, 3, 10)));
    }

    @Test
    void testCase2() {
        assertEquals(Set.of(2, 4, 6, 8, 10, 14), new HashSet<>(SOLUTION.powerfulIntegers(3, 5, 15)));
    }
}
