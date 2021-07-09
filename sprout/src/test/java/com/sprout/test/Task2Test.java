package com.sprout.test;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Task2Test {
    private static final Task2 SOLUTION = new Task2();

    @Test
    public void testCase1() {
        assertEquals(1, SOLUTION.solution(new int[]{1, -2, -3, 5}));
    }

    @Test
    public void testCase2() {
        assertEquals(-1, SOLUTION.solution(new int[]{1, 2, 3, -5}));
    }

    @Test
    public void testCase3() {
        assertEquals(0, SOLUTION.solution(new int[]{1, 2, 0, -5}));
    }
}
