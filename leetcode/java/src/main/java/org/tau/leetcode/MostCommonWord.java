package org.tau.leetcode;

import java.util.Arrays;
import java.util.Collection;
import java.util.HashSet;
import java.util.Locale;
import java.util.Map;
import java.util.NavigableMap;
import java.util.TreeMap;
import java.util.function.Function;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class MostCommonWord {
    private static final Pattern PATTERN = Pattern.compile("[ \"!?',;.]+");

    public String mostCommonWord(String paragraph, String[] banned) {
        Collection<String> bannedSet = new HashSet<>(Arrays.asList(banned));
        Map<String, Long> counts = Arrays.stream(PATTERN.split(paragraph))
                .map(str -> str.toLowerCase(Locale.ENGLISH))
                .filter(str -> !bannedSet.contains(str))
                .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));

        NavigableMap<Long, String> sorted = new TreeMap<>();
        counts.entrySet().forEach((e -> sorted.put(e.getValue(), e.getKey())));
        return sorted.lastEntry().getValue();
    }
}
