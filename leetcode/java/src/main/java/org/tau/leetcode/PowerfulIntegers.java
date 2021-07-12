package org.tau.leetcode;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.NavigableSet;
import java.util.TreeSet;
import java.util.stream.Collectors;

class PowerfulIntegers {
    public List<Integer> powerfulIntegers(int x, int y, int bound) {
        Collection<Integer> xPows = new TreeSet<>();
        for (int i = 0, xPow = 1; xPow <= bound; i++, xPow = (int) StrictMath.pow(x, i)) {
            xPows.add(xPow);
        }

        NavigableSet<Integer> yPows = new TreeSet<>();
        for (int i = 0, yPow = 1; yPow <= bound; i++, yPow = (int) StrictMath.pow(y, i)) {
            yPows.add(yPow);

        }

        List<Integer> result = new ArrayList<>();
        for (int xPow : xPows) {
            yPows.headSet(bound - xPow, true)
                    .stream()
                    .map(yPow -> yPow + xPow)
                    .forEach(result::add);
        }
        return result.stream()
                .distinct()
                .collect(Collectors.toList());
    }
}