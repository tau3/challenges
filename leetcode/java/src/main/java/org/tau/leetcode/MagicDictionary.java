package org.tau.leetcode;

import java.util.Arrays;
import java.util.Collection;

class MagicDictionary {
    private Collection<String> dictionary;

    public MagicDictionary() {
    }

    public void buildDict(String[] dictionary) {
        this.dictionary = Arrays.asList(dictionary);
    }

    public boolean search(String searchWord) {
        return dictionary.stream()
                .anyMatch(w -> isComaptible(w, searchWord));
    }

    private static boolean isComaptible(String left, String right) {
        int length = left.length();
        if (length != right.length()) {
            return false;
        }

        int diffs = 0;
        for (int i = 0; i < length; i++) {
            char l = left.charAt(i);
            char r = right.charAt(i);
            if (l != r) {
                diffs++;
            }
            if (diffs > 1) {
                break;
            }
        }
        return diffs == 1;
    }
}