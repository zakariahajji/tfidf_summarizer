use super::frequency;
use std::collections::HashMap;

pub fn inverse_document_frequency(
    df: &HashMap<String, usize>,
    doc_count: usize,
) -> HashMap<String, f64> {
    let mut idf = HashMap::new();
    for (term, count) in df.iter() {
        let idf_value = (doc_count as f64 / *count as f64).ln();
        idf.insert(term.clone(), idf_value);
    }
    idf
}

pub fn tf_idf(tokens: Vec<String>, idf: &HashMap<String, f64>) -> HashMap<String, f64> {
    let tf = frequency::term_frequency(tokens);
    let mut tf_idf_scores = HashMap::new();
    for (term, freq) in tf {
        let idf_score = idf.get(&term).unwrap_or(&0.0);
        tf_idf_scores.insert(term, freq * idf_score);
    }
    tf_idf_scores
}
