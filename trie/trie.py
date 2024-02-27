#!/home/jpal/mambaforge/bin/python
import json
from collections import defaultdict

class Node:

    def __init__(self):
        self.end_of_word: bool = False
        self.children: dict = {}

class Trie:
    
    def __init__(self):
        self.root = Node()
    
    def insert(self, word: str) -> None:
        node = self.root

        for w in word:
            if w not in node.children:
                node.children[w] = Node()

            node = node.children[w]
        
        node.end_of_word = True
    
    def search(self, word: str) -> bool:
        node = self.root

        for w in word:
            if w not in node.children:
                return False
            else:
                node = node.children[w]
        
        return node.end_of_word

def main():
    trie = Trie()

    with open('test.json') as f:
        words = json.load(f)
    
    l = len(words)
    half_words = words[:l]

    for word in half_words:
        # print(f'Insert {word}')
        trie.insert(word)
    
    for i, w in enumerate(words):
        if i < l:
            assert trie.search(w)
        else:
            assert not trie.search(w)

if __name__ == "__main__":
    main()