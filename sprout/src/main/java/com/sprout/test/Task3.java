package com.sprout.test;

import java.util.Comparator;
import java.util.HashMap;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Queue;

public class Task3 {
    public int solution(int[] A) {
        int maxSum = -1;
        Map<Integer, Queue<Integer>> cache = new HashMap<>();
        for (int val : A) {
            int sum = sumOfNumbers(val);
            if (cache.containsKey(sum)) {
                Queue<Integer> candidates = cache.get(sum);
                candidates.add(val);
                maxSum = Math.max(maxSum, sum);
            } else {
                Queue<Integer> candidates = new PriorityQueue<>(Comparator.reverseOrder());
                candidates.add(val);
                cache.put(sum, candidates);
            }
        }

        return cache.values()
                .stream()
                .filter(queue -> queue.size() > 1)
                .map(queue -> queue.remove() + queue.remove())
                .max(Comparator.naturalOrder())
                .orElse(-1);
    }

    private static int sumOfNumbers(int a) {
        int result = 0;
        while (a > 0) {
            int digit = a % 10;
            result += digit;
            a /= 10;
        }
        return result;
    }
}
