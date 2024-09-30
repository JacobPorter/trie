use std::collections::HashMap;

struct Trie {
    children: HashMap<char, Trie>,
    count: HashMap<char, i32>,
    leaves: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            count: HashMap::new(),
            leaves: 0,
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut current = self;
        for c in word.chars() {
            current.count.entry(c).and_modify(|e| *e += 1).or_insert(1);
            current = current.children.entry(c).or_insert(Trie::new());
        }
        current.leaves += 1;
    }
    
    fn count_words_equal_to(&self, word: String) -> i32 {
        let mut current = self;
        for c in word.chars() {
            if !current.children.contains_key(&c) {
                return 0;
            }
            current = current.children.get(&c).unwrap();
        }
        current.leaves
    }
    
    fn count_words_starting_with(&self, prefix: String) -> i32 {
        let mut current = self;
        let mut count = 0i32;
        for c in prefix.chars() {
            if!current.children.contains_key(&c) {
                return 0;
            }
            count = *current.count.get(&c).unwrap();
            current = current.children.get(&c).unwrap();
        }
        count
    }
    
    fn erase(&mut self, word: String) {
        let mut current = self;
        for c in word.chars() {
            current.count.entry(c).and_modify(|e| *e -= 1);
            // If the count becomes zero, remove the node from the children
            if *current.count.get(&c).unwrap() == 0 {
                current.children.remove(&c);
                return;
            }
            current = current.children.get_mut(&c).unwrap();
        }
        if current.leaves > 0 {
            current.leaves -= 1;
        }
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("hello".to_string());
    trie.insert("world".to_string());
    trie.insert("rust".to_string());

    println!("Words equal to 'hello': {}", trie.count_words_equal_to("hello".to_string()));
    println!("Words equal to 'world': {}", trie.count_words_equal_to("world".to_string()));
    println!("Words starting with 'wor': {}", trie.count_words_starting_with("wor".to_string()));

    trie.erase("hello".to_string());
    println!("Words equal to 'hello' after erasing: {}", trie.count_words_equal_to("hello".to_string()));
}

// /**
//  * Your Trie object will be instantiated and called as such:
//  * let obj = Trie::new();
//  * obj.insert(word);
//  * let ret_2: i32 = obj.count_words_equal_to(word);
//  * let ret_3: i32 = obj.count_words_starting_with(prefix);
//  * obj.erase(word);
//  */