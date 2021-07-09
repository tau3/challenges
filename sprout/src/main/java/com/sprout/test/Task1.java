package com.sprout.test;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Function;
import java.util.stream.Collectors;

public class Task1 {
    public int solution(int N, int[] A, int[] B) {
        List<Integer> sortedVertexes = sortVertexesByFrequency(A, B);
        Map<Integer, Integer> vertexToWeight = assignWeights(N, sortedVertexes);
        return calculateSum(A, B, vertexToWeight);
    }

    private int calculateSum(int[] A, int[] B, Map<Integer, Integer> vertexToValue) {
        int result = 0;
        for (int i = 0; i < A.length; i++) {
            int start = A[i];
            int end = B[i];
            result += vertexToValue.get(start) + vertexToValue.get(end);
        }
        return result;
    }

    private List<Integer> sortVertexesByFrequency(int[] A, int[] B) {
        List<Integer> vertexAppearances = new ArrayList<>();

        vertexAppearances.addAll(arrayToList(A));
        vertexAppearances.addAll(arrayToList(B));
        vertexAppearances.sort(Comparator.reverseOrder());

        Map<Integer, Long> vertexToAppearances = vertexAppearances.stream()
                .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));

        return vertexAppearances.stream()
                .distinct()
                .sorted(Comparator.comparing(vertexToAppearances::get).reversed())
                .toList();
    }

    private Map<Integer, Integer> assignWeights(int N, List<Integer> sortedVertexes) {
        Map<Integer, Integer> vertexToValue = new HashMap<>();
        int currentWeight = N;
        for (int vertex : sortedVertexes) {
            vertexToValue.put(vertex, currentWeight);
            currentWeight--;
        }
        return vertexToValue;
    }

    private static List<Integer> arrayToList(int[] array) {
        return Arrays.stream(array).boxed().toList();
    }
}
