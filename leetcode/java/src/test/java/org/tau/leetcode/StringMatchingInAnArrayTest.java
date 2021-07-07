package org.tau.leetcode;

import org.junit.jupiter.api.Test;

import java.util.Collections;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class StringMatchingInAnArrayTest {
    private static final StringMatchingInAnArray SOLUTION = new StringMatchingInAnArray();

    @Test
    void testCase1() {
        assertEquals(List.of("as", "hero"),
                SOLUTION.stringMatching(new String[]{"mass", "as", "hero", "superhero"}));
    }

    @Test
    void testCase2() {
        assertEquals(List.of("code", "et"),
                SOLUTION.stringMatching(new String[]{"leetcode", "et", "code"}));
    }

    @Test
    void testCase3() {
        assertEquals(Collections.emptyList(),
                SOLUTION.stringMatching(new String[]{"blue", "green", "bu"}));
    }

    @Test
    void testCase4() {
        assertEquals(List.of("od", "leetcode", "am"),
                SOLUTION.stringMatching(new String[]{"leetcoder", "leetcode", "od", "hamlet", "am"}));
    }
}
