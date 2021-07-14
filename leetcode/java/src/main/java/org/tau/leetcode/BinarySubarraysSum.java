package org.tau.leetcode;

import java.util.HashMap;
import java.util.Map;

class BinarySubarraysSum {
    // O(n^2) space -> too bad
    public int numSubarraysWithSumDp(int[] nums, int goal) {
        int[][] dp = new int[nums.length][nums.length];
        int result = 0;
        for (int start = 0; start < nums.length; start++) {
            for (int end = start; end < nums.length; end++) {
                if (start == end) {
                    dp[start][end] = nums[start];
                } else {
                    dp[start][end] = nums[end] + dp[start][end - 1];
                }
                if (dp[start][end] == goal) {
                    result++;
                }
            }
        }
        return result;
    }

    public int numSubarraysWithSum(int[] nums, int goal) {
        int[] prefixes = new int[nums.length + 1];
        for (int i = 0; i < nums.length; i++) {
            prefixes[i + 1] = prefixes[i] + nums[i];
        }

        int result = 0;
        Map<Integer, Integer> counts = new HashMap<>();
        for (int prefix : prefixes) {
            result += counts.getOrDefault(prefix, 0);
            counts.compute(prefix + goal, (key, oldValue) -> oldValue == null ? 1 : oldValue + 1);
        }
        return result;
    }
}