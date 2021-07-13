package org.tau.leetcode;

class BinarySubarraysSum {
    public int numSubarraysWithSum(int[] nums, int goal) {
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
}