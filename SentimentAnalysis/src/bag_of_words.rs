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

    pub fn zipf_cut(mut self, min_count: u32, stop_words: &HashSet<String>) -> Self {
        self.0
            .retain(|k, v| *v > min_count && k.len() > 1 && !stop_words.contains(k));

        self
    }

    pub fn build_dictionary(positive_bow: BagOfWords, negative_bow: BagOfWords, bias: f32) -> (HashSet<String>, HashSet<String>) {
        todo!()
    }
}

impl Deref for BagOfWords {
    type Target = HashMap<String, u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for BagOfWords {
    fn from(value: String) -> Self {
        let mut map = HashMap::new();
        map.insert(value, 1);
        BagOfWords(map)
    }
}

impl From<&[&str]> for BagOfWords {
    fn from(texts: &[&str]) -> Self {
        texts
            .iter()
            .flat_map(|s| s.split(|c: char| !c.is_alphabetic() || c.is_whitespace()))
            .filter(|s| !s.is_empty())
            .map(str::to_lowercase)
            .fold(BagOfWords::empty(), BagOfWords::add_word)
    }
}
