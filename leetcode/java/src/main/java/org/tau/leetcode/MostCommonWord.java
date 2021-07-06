package org.tau.leetcode;

import java.util.Arrays;
import java.util.Collection;
import java.util.HashSet;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.NavigableMap;
import java.util.TreeMap;
import java.util.function.Function;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

@SuppressWarnings({"PublicConstructor", "unused"})
public class MostCommonWord {
    private static final Pattern PATTERN = Pattern.compile("[ \"!?',;.]");

    @SuppressWarnings({"unused", "MethodMayBeStatic", "DesignForExtension"})
    public String mostCommonWord(@SuppressWarnings("TypeMayBeWeakened") String paragraph, String[] banned) {
        Collection<String> bannedSet = new HashSet<>(Arrays.asList(banned));
        List<String> strings = Arrays.stream(PATTERN.split(paragraph))
                .map(str -> str.toLowerCase(Locale.ENGLISH))
                .filter(str -> !bannedSet.contains(str))
                .toList();

        NavigableMap<Long, String> sorted = new TreeMap<>();
        Map<String, Long> counts = strings.stream()
                .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));
        counts.entrySet().forEach((e -> sorted.put(e.getValue(), e.getKey())));
        return sorted.firstEntry().getValue();
    }
}
