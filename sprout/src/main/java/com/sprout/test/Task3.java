package com.sprout.test;

import java.util.HashMap;
import java.util.Map;

public class Task3 {
    public int solution(int[] A) {
        int result = -1;
        Map<Integer, Integer> cache = new HashMap<>();
        for (int val : A) {
            int sum = sumOfNumbers(val);
            if (cache.containsKey(sum)) {
                result = Math.max(result, sum);
            } else {
                cache.put(sum, val);
            }
        }
        return result;
    }

    private static int sumOfNumbers(int a) {
        int result = 0;
        while (a > 0) {
            int digit = a % 10;
            result += digit;
            a /= 10;
        }
        System.out.println(a + " " + result);
        return result;
    }
}
