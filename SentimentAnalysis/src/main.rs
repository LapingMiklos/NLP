use std::collections::HashSet;

use bag_of_words::BagOfWords;
use csv::{Reader, Writer};
use dictionary::BinarySentimentDictionary;
use serde::Deserialize;

pub mod bag_of_words;
pub mod dictionary;

static TRAIN_DATA: &'static str = "./data/train.csv";
static STOP_WORDS: &'static str = "./data/stop_words.csv";
static TEST_DATA: &'static str = "./data/test.csv";
static PREDICTIONS: &'static str = "./data/predictions.csv";

#[derive(Debug, Deserialize, Clone)]
struct Review {
    #[serde(rename = "ID")]
    id: i32,
    review: String,
    decision: Option<i32>,
}

impl Review {
    fn is_positive(&self) -> bool {
        match self.decision {
            Some(1) => true,
            _ => false,
        }
    }

    fn get_text(&self) -> &str {
        &self.review
    }
}

fn main() {
    let mut reader =
        Reader::from_path(TRAIN_DATA).expect(&format!("Expexting train data: {}", TRAIN_DATA));
    let mut test_reader =
        Reader::from_path(TRAIN_DATA).expect(&format!("Expexting test data: {}", TEST_DATA));
    let mut sw_reader =
        Reader::from_path(STOP_WORDS).expect(&format!("Expexting stop words: {}", STOP_WORDS));

    let reviews: Vec<Review> = reader.deserialize().filter_map(Result::ok).collect();
    let test_reviews: Vec<Review> = test_reader.deserialize().flat_map(Result::ok).collect();

    let stop_words: HashSet<String> = sw_reader.deserialize().filter_map(Result::ok).collect();

    let (positive_reviews, negative_reviews): (Vec<_>, Vec<_>) =
        reviews.iter().partition(|r| r.is_positive());
    let positive_reviews = positive_reviews
        .into_iter()
        .map(Review::get_text)
        .collect::<Vec<_>>();
    let negative_reviews = negative_reviews
        .into_iter()
        .map(Review::get_text)
        .collect::<Vec<_>>();

    let positive_bow = BagOfWords::from(positive_reviews.as_slice());
    let negative_bow = BagOfWords::from(negative_reviews.as_slice());

    let dict = BinarySentimentDictionary::build(positive_bow, negative_bow, &stop_words, 0.5);

    dbg!(&dict);

    let correct_guesses = reviews
        .iter()
        .map(|r| (dict.classify(r.get_text()), r.is_positive()))
        .map(|(pred, label)| if pred == label { 1 } else { 0 })
        .sum::<u32>();

    println!(
        "{}/{} = {}",
        correct_guesses,
        reviews.len(),
        correct_guesses as f32 / reviews.len() as f32
    );

    let mut writer = Writer::from_path(PREDICTIONS)
        .expect(&format!("Could not open predictions file: {}", PREDICTIONS));
    let prediction = test_reviews
        .iter()
        .map(|r| (r.id, if dict.classify(&r.review) { 1 } else { -1 }))
        .collect::<Vec<_>>();

    let _ = writer.serialize(("ID", "decision"));
    for p in prediction {
        let _ = writer.serialize(p);
    }
}
