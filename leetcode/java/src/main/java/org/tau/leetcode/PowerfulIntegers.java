package org.tau.leetcode;

import java.util.ArrayList;
import java.util.Collection;
import java.util.HashSet;
import java.util.List;
import java.util.stream.Collectors;

class PowerfulIntegers {
    public List<Integer> powerfulIntegers(int x, int y, int bound) {
        Collection<Integer> result = new HashSet<>();
        int i = 1, j = 1;
        do {
            do {
                int candidate = i + j;
                if (candidate <= bound) {
                    result.add(candidate);
                }
                j *= y;
            }
            while ((y != 1) && (j < bound));
            i *= x;
            j = 1;
        }
        while ((x != 1) && (i < bound));

        return new ArrayList<>(result);
    }
}