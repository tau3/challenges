package com.sprout.test;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Task3Test {
    private static final Task3 SOLUTION = new Task3();

    @Test
    public void testCase1() {
        assertEquals(93, SOLUTION.solution(new int[]{51, 71, 17, 42}));
    }
}
