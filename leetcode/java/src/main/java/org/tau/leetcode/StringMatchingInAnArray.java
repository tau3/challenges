package org.tau.leetcode;

import java.util.ArrayList;
import java.util.List;

public class StringMatchingInAnArray {
    public List<String> stringMatching(String[] words) {
        List<String> result = new ArrayList<>();
        for (int i = 0; i < words.length; i++) {
            for (int j = i + 1; j < words.length; j++) {
                if (words[i].contains(words[j])) {
                    result.add(words[j]);
                } else if (words[j].contains(words[i])) {
                    result.add(words[i]);
                }
            }
        }
        return result;
    }
}
