package com.sprout.test;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Task3Test {
    private static final Task3 SOLUTION = new Task3();

    @Test
    public void testCase1() {
        assertEquals(93, SOLUTION.solution(new int[]{51, 71, 17, 42}));
    }

    @Test
    public void testCase2() {
        assertEquals(102, SOLUTION.solution(new int[]{42, 33, 60}));
    }

    @Test
    public void testCase2_1() {
        assertEquals(102, SOLUTION.solution(new int[]{33, 42, 60}));
    }

    @Test
    public void testCase3() {
        assertEquals(-1, SOLUTION.solution(new int[]{51, 32, 43}));
    }
}

