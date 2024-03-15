use std::collections::{HashMap, HashSet};

#[derive(PartialEq)]
struct Counter(HashMap<char, u32>);

struct Word<'a>(&'a str, Counter);

impl<'a> Word<'a> {
    fn new(word: &'a str) -> Self {
        Word(word, Counter::new(word))
    }

    fn is_ana(&self, other: &Word) -> Option<&'a str> {
        (other.0.to_lowercase() != self.0.to_lowercase() && other.1 == self.1).then_some(self.0)
    }
}

impl Counter {
    fn new(word: &str) -> Self {
        let mut map: HashMap<char, u32> = HashMap::new();

        word.to_lowercase().chars().for_each(|c| {
            map.entry(c).and_modify(|n| *n += 1).or_default();
        });

        Self(map)
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let cmp_word = Word::new(word);

    possible_anagrams
        .iter()
        .filter_map(|&w| Word::new(w).is_ana(&cmp_word))
        .collect()
}
