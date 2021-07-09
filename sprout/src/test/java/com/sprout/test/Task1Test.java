package com.sprout.test;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Task1Test {
    private static final Task1 SOLUTION = new Task1();

    @Test
    public void testCase1() {
        assertEquals(31, SOLUTION.solution(5, new int[]{2, 2, 1, 2}, new int[]{1, 3, 4, 4}));
    }

    @Test
    public void testCase2() {
        assertEquals(5, SOLUTION.solution(3, new int[]{1}, new int[]{3}));
    }

    @Test
    public void testCase3() {
        assertEquals(10, SOLUTION.solution(4, new int[]{1, 3}, new int[]{2, 4}));
    }
}
