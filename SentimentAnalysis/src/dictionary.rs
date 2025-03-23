use std::{collections::{HashMap, HashSet}, error::Error};

use csv::Writer;

use crate::bag_of_words::{to_words, BagOfWords};

#[derive(Debug)]
pub struct BinarySentimentDictionary(HashMap<String, bool>);

impl BinarySentimentDictionary {
    pub fn build(mut positive_bow: BagOfWords, mut negative_bow: BagOfWords, stop_words: &HashSet<String>) -> Self {
        positive_bow.zipf_cut(10, stop_words);
        negative_bow.zipf_cut(5, stop_words);

        let words: HashSet<&String> = positive_bow.keys().chain(negative_bow.keys()).collect();

        BinarySentimentDictionary(
            words
            .into_iter()
            .filter_map(|word| {
                let poz = positive_bow.get(word) as f32;
                let neg = negative_bow.get(word) as f32;

                let ratio = poz / (poz + neg);

                if ratio > 0.9 {
                    Some((word.clone(), true))
                } else if ratio < 0.1 {
                    Some((word.clone(), false))
                } else {
                    None
                }
            })
            .collect(), 
        )
    }

    pub fn classify(&self, text: &str) -> bool {
        let (positive_positive, negative_words) = to_words(text)
            .into_iter()
            .filter_map(|w| self.0.get(&w))
            .partition::<Vec<bool>, _>(|l| **l);

        positive_positive.len() > negative_words.len()
    }

    pub fn export(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut writer = Writer::from_path(path)?;
        
        for (word, _) in self.0.iter().filter(|(_, label)| **label) {
            writer.serialize((word, true))?;
        }

        for (word, _) in self.0.iter().filter(|(_, label)| !*label) {
            writer.serialize((word, false))?;
        }

        Ok(())
    }
}