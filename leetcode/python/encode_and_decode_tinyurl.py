import random
import string

SYMBOLS = string.ascii_lowercase + string.digits
LENGTH = 6


def generate_key():
    return ''.join(random.choice(SYMBOLS) for _ in range(LENGTH))


class Codec:
    def __init__(self):
        self.cache = {}

    def encode(self, longUrl):
        key = generate_key()
        result = 'http://tinyurl.com/' + key
        self.cache[result] = longUrl
        return result

    def decode(self, shortUrl):
        return self.cache[shortUrl]


def main():
    codec = Codec()
    url = 'https://leetcode.com/problems/design-tinyurl'
    print(codec.decode(codec.encode(url)))


if __name__ == '__main__':
    main()
