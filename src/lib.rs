pub mod frequency;
pub mod tfidf;
pub mod tokenizer;

pub use frequency::{document_frequency, term_frequency};
pub use tfidf::{inverse_document_frequency, tf_idf};
pub use tokenizer::tokenize;

/// Document struct that could be used externally.
pub struct Document {
    pub id: usize,
    pub text: String,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tfidf::inverse_document_frequency;
    use std::collections::HashMap; // Add this to use HashMap in the tests. // Import the function from the tfidf module.

    #[test]
    fn test_document_frequency() {
        // Example documents
        let doc1 = tokenize("The quick brown fox");
        let doc2 = tokenize("jumps over the lazy dog");
        let docs = vec![doc1, doc2];
        let df = document_frequency(&docs);

        // Expected document frequencies
        let expected_df: HashMap<String, usize> = [
            ("the", 2),
            ("quick", 1),
            ("brown", 1),
            ("fox", 1),
            ("jumps", 1),
            ("over", 1),
            ("lazy", 1),
            ("dog", 1),
        ]
        .iter()
        .map(|(k, v)| (String::from(*k), *v))
        .collect();

        assert_eq!(df, expected_df);
    }

    #[test]
    fn test_inverse_document_frequency() {
        let df: HashMap<String, usize> = [
            ("the", 2),
            ("quick", 1),
            ("brown", 1),
            ("fox", 1),
            ("jumps", 1),
            ("over", 1),
            ("lazy", 1),
            ("dog", 1),
        ]
        .iter()
        .map(|(k, v)| (String::from(*k), *v))
        .collect();
        let idf = inverse_document_frequency(&df, 2);

        // Test if IDF for a term present in both documents is lower than for a term present in one
        assert!(idf["the"] < idf["quick"]);
    }

    #[test]
    fn test_tf_idf() {
        let tokens = tokenize("The quick brown fox jumps over the lazy dog");
        let df = document_frequency(&vec![tokens.clone()]);
        let idf = inverse_document_frequency(&df, 1); // Only one document
        let tf_idf_scores = tf_idf(tokens, &idf);

        // We only have one document so the IDF for every term will be 0
        // the TF-IDF scores should be 0 for each term
        for score in tf_idf_scores.values() {
            assert_eq!(*score, 0.0);
        }
    }
}
