use std::collections::HashSet;

use bag_of_words::BagOfWords;
use csv::Reader;
use serde::Deserialize;

pub mod bag_of_words;

static TRAIN_DATA: &'static str = "./data/train.csv";
static STOP_WORDS: &'static str = "./data/stop_words.csv";
static TEST_DATA: &'static str = "./data/test.csv";

#[derive(Debug, Deserialize)]
struct Review {
    #[serde(rename = "ID")]
    id: i32,
    review: String,
    decision: Option<i32>,
}

impl Review {
    fn is_positive(&self) -> bool{
        match self.decision {
            Some(1) => true,
            _ => false
        }
    }

    fn get_text(&self) -> &str {
        &self.review
    }
}

fn main() {
    let mut reader = Reader::from_path(TRAIN_DATA).expect(&format!("Expexting train data: {}", TRAIN_DATA));
    let mut sw_reader = Reader::from_path(STOP_WORDS).expect(&format!("Expexting train data: {}", STOP_WORDS));

    let reviews: Vec<Review> = reader.deserialize().filter_map(Result::ok).collect();
    let stop_words: HashSet<String> = sw_reader.deserialize().filter_map(Result::ok).collect();

    let (positive_reviews, negative_reviews): (Vec<_>, Vec<_>) = reviews.into_iter().partition(Review::is_positive);
    let positive_reviews = positive_reviews.iter().map(Review::get_text).collect::<Vec<_>>();
    let negative_reviews = negative_reviews.iter().map(Review::get_text).collect::<Vec<_>>();

    let positive_bow= BagOfWords::from(positive_reviews.as_slice()).zipf_cut(10, &stop_words);
    let negative_bow= BagOfWords::from(negative_reviews.as_slice()).zipf_cut(10, &stop_words);

    let (positive_dictionary, negative_dictionary) = BagOfWords::build_dictionary(positive_bow, negative_bow, 0.8);
}
