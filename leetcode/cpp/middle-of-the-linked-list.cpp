#define CATCH_CONFIG_MAIN
#include <map>

#include <algorithm>
#include <catch2/catch.hpp>

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
  public:
    ListNode *middleNode(ListNode *head) {
        ListNode *tortoise = head;
        ListNode *hare = head;
        while (hare != nullptr && hare->next != nullptr) {
            tortoise = tortoise->next;
            hare = hare->next->next;
        }
        return tortoise;
    }
};

void print_node(const ListNode *node) {
    while (node) {
        std::cout << node->val << " ";
        node = node->next;
    }
    std::cout << std::endl;
}

ListNode *vector_to_list(std::vector<int> values) {
    std::reverse(values.begin(), values.end());
    ListNode *prev = nullptr;
    ListNode *current = nullptr;
    for (int value : values) {
        current = new ListNode(value);
        current->next = prev;
        prev = current;
    }
    // print_node(current);
    return current;
}

TEST_CASE("valid middle", "[P876]") {
    const std::map<std::vector<int>, int> cases = {
        {{1}, 1},
        {{1, 2}, 2},
        {{1, 2, 3}, 2},
        {{1, 2, 3, 4}, 3},
        {{1, 2, 3, 4, 5}, 3},
        {{1, 2, 3, 4, 5, 6}, 4},
        {{1, 2, 3, 4, 5, 6, 7}, 4},
        {{1, 2, 3, 4, 5, 6, 7, 8}, 5},
        {{1, 2, 3, 4, 5, 6, 7, 8, 9}, 5}};

    Solution solution;
    for (auto entry : cases) {
        const std::vector<int> values = entry.first;
        const int expected = entry.second;

        ListNode *node = vector_to_list(values);
        const ListNode *actual = solution.middleNode(node);

        REQUIRE(expected == actual->val);
    }
}
