package org.tau.leetcode;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class StringMatchingInAnArray {
    public List<String> stringMatching(String[] words) {
        Set<String> result = new HashSet<>();
        for (int i = 0; i < words.length; i++) {
            for (int j = i + 1; j < words.length; j++) {
                if (words[i].contains(words[j])) {
                    result.add(words[j]);
                } else if (words[j].contains(words[i])) {
                    result.add(words[i]);
                }
            }
        }
        return new ArrayList<>(result);
    }
}
