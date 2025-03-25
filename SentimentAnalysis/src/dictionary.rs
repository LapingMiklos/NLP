use std::{collections::{HashMap, HashSet}, error::Error};

use csv::Writer;

use crate::bag_of_words::{to_words, BagOfWords};

#[derive(Debug)]
pub struct BinarySentimentDictionary(HashMap<String, bool>);

impl BinarySentimentDictionary {
    pub fn build(
        mut positive_bow: BagOfWords, 
        mut negative_bow: BagOfWords, 
        stop_words: &HashSet<String>,
        min_word_freq: u32, 
        acceptance_ratio_poz: f64, 
        acceptance_ratio_neg: f64
    ) -> Self {
        positive_bow.remove_words(stop_words);
        negative_bow.remove_words(stop_words);

        let positive_words = positive_bow.iter().map(|(_, v)| v).sum::<u32>() as f64;
        let negative_words = negative_bow.iter().map(|(_, v)| v).sum::<u32>() as f64;

        let uncommon_words = BagOfWords::merge(&positive_bow, &negative_bow)
            .0
            .into_iter()
            .filter_map(|(word, freq)| if freq < min_word_freq { Some(word) } else { None })
            .collect::<HashSet<_>>();

        positive_bow.remove_words(&uncommon_words);
        negative_bow.remove_words(&uncommon_words);

        let words: HashSet<&String> = positive_bow.keys().chain(negative_bow.keys()).collect();

        BinarySentimentDictionary(
            words
            .into_iter()
            .filter_map(|word| {
                let poz = positive_bow.get(word) as f64 / positive_words;
                let neg = negative_bow.get(word) as f64 / negative_words;

                let ratio = poz / (poz + neg);

                if ratio > acceptance_ratio_poz {
                    Some((word.clone(), true))
                } else if ratio < (1. - acceptance_ratio_neg) {
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