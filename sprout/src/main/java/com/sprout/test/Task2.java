package com.sprout.test;

public class Task2 {
    public int solution(int[] A) {
        int negatives = 0;
        for (int val : A) {
            if (val == 0) {
                return 0;
            }
            if (val < 0) {
                negatives++;
            }
        }
        return (negatives % 2 == 0) ? 1 : -1;
    }
}
