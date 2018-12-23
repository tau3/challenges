class Solution(object):
    def numUniqueEmails(self, emails):
        return len(set(map(self._normalize, emails)))

    def _normalize(self, email):
        (local, domain) = email.split('@')
        plus_pos = local.find("+")
        if plus_pos != -1:
            local = local[:plus_pos]
        local = local.replace(".", "")
        result = local + '@' + domain
        return result


def main():
    solution = Solution()
    print(solution.numUniqueEmails(["test.email+alex@leetcode.com",
                                    "test.e.mail+bob.cathy@leetcode.com",
                                    "testemail+david@lee.tcode.com"]))


if __name__ == '__main__':
    main()
