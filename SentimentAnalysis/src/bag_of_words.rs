use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

#[derive(Debug)]
pub struct BagOfWords(HashMap<String, u32>);

impl BagOfWords {
    pub fn empty() -> Self {
        BagOfWords(HashMap::new())
    }

    pub fn add_word(mut self, word: String) -> Self {
        self.0.entry(word).and_modify(|v| *v += 1).or_insert(1);
        self
    }

    pub fn zipf_cut(&mut self, min_count: u32, stop_words: &HashSet<String>) {
        self.0
            .retain(|k, v| *v > min_count && k.len() > 1 && !stop_words.contains(k));
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

impl From<&str> for BagOfWords {
    fn from(value: &str) -> Self {
        value
            .split(not_alpha)
            .filter(|s | !s.is_empty())
            .map(str::to_lowercase)
            .fold(BagOfWords::empty(), BagOfWords::add_word)
    }
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
