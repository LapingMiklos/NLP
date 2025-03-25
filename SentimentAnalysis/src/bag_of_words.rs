use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

#[derive(Debug, Clone)]
pub struct BagOfWords(pub HashMap<String, u32>);

impl BagOfWords {
    pub fn empty() -> Self {
        BagOfWords(HashMap::new())
    }

    pub fn add_word(self, word: String) -> Self {
        self.add_words(word, 1)
    }

    pub fn add_words(mut self, word: String, amount: u32) -> Self {
        self.0.entry(word).and_modify(|v| *v += amount).or_insert(amount);
        self
    }

    pub fn merge(&self, other: &Self) -> Self {
        self
            .clone()
            .0
            .into_iter()
            .fold(other.clone(), |acc, (word, amount)| acc.add_words(word, amount))
    }

    pub fn remove_words(&mut self, words: &HashSet<String>) {
        self.0
            .retain(|k, _| !words.contains(k));
    }
    
    pub fn get(&self, word: &str) -> u32 {
        self.0.get(word).unwrap_or(&0).clone()
    }
}

impl Deref for BagOfWords {
    type Target = HashMap<String, u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn not_alpha(c: char) -> bool {
    !c.is_alphabetic()
}

pub fn to_words(text: &str) -> Vec<String> {
    text
        .split(not_alpha)
        .filter(|s | !s.is_empty())
        .map(str::to_lowercase)
        .collect()
}

impl From<&[&str]> for BagOfWords {
    fn from(texts: &[&str]) -> Self {
        texts
            .iter()
            .flat_map(|s| s.split(not_alpha))
            .filter(|s| !s.is_empty())
            .map(str::to_lowercase)
            .fold(BagOfWords::empty(), BagOfWords::add_word)
    }
}
