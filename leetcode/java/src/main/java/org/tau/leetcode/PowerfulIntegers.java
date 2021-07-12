package org.tau.leetcode;

import java.util.ArrayList;
import java.util.Collection;
import java.util.HashSet;
import java.util.List;

class PowerfulIntegers {
    private static final int MAX_POW = 20;

    public List<Integer> powerfulIntegers(int x, int y, int bound) {
        int[] xPows = new int[MAX_POW];
        xPows[0] = 1;
        int[] yPows = new int[MAX_POW];
        yPows[0] = 1;

        Collection<Integer> result = new HashSet<>();
        outerloop:
        for (int i = 0; i < MAX_POW; i++) {
            for (int j = 0; j < MAX_POW; j++) {
                if (xPows[i] == 0) {
                    xPows[i] = (int) StrictMath.pow(x, i);
                }
                if (xPows[i] >= bound) {
                    break outerloop;
                }

                if (yPows[j] == 0) {
                    yPows[j] = (int) StrictMath.pow(y, j);
                }

                int candidate = xPows[i] + yPows[j];
                if (candidate <= bound) {
                    result.add(candidate);
                } else {
                    break;
                }
            }
        }

        return new ArrayList<>(result);
    }
}