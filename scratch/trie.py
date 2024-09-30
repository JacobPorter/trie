class TrieNode:
    def __init__(self):
        self.children = [None] * 26
        self.count = [0] * 26
        self.isLeaf = 0

class Trie:

    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        """
        Inserts the string word into the trie.
        """
        current = self.root
        for letter in word:
            index = ord(letter) - ord('a')
            current.count[index] += 1
            if not current.children[index]:
                current.children[index] = TrieNode()
            current = current.children[index]
        current.isLeaf += 1

    def countWordsEqualTo(self, word: str) -> int:
        """
        Returns the number of instances of the string word in the trie.
        """
        current = self.root
        for letter in word:
            index = ord(letter) - ord('a')
            if not current.children[index]:
                return 0
            current = current.children[index]
        if current.isLeaf and current:
            return current.isLeaf
        return 0

    def countWordsStartingWith(self, prefix: str) -> int:
        """
        Returns the number of strings in the trie that have the string prefix as a prefix.
        """
        current = self.root
        count = 0
        for letter in prefix:
            index = ord(letter) - ord('a')
            if not current.children[index]:
                return 0
            count = current.count[index]
            current = current.children[index]
        return count

    def erase(self, word: str) -> None:
        """
        Erases the string word from the trie.
        """
        current = self.root
        for letter in word:
            index = ord(letter) - ord('a')
            current.count[index] -= 1
            if not current.count[index]:
                current.children[index] = None
                return
            current = current.children[index]
        if current.isLeaf:
            current.isLeaf -= 1

# Your Trie object will be instantiated and called as such:
# obj = Trie()
# obj.insert(word)
# param_2 = obj.countWordsEqualTo(word)
# param_3 = obj.countWordsStartingWith(prefix)
# obj.erase(word)