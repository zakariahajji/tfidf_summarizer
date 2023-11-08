use std::collections::{HashMap, HashSet};

/// Caalculates the document frequency of terms in a collection of documents.

pub fn document_frequency(docs: &[Vec<String>]) -> HashMap<String, usize> {
    let mut df = HashMap::new();
    for doc in docs {
        let unique_terms: HashSet<_> = doc.iter().cloned().collect();
        for term in unique_terms {
            *df.entry(term).or_insert(0) += 1;
        }
    }
    df
}

pub fn term_frequency(tokens: Vec<String>) -> HashMap<String, f64> {
    let mut term_counts = HashMap::new();
    let total_terms = tokens.len() as f64;

    // counttt the occurrences of each term
    for token in tokens {
        *term_counts.entry(token).or_insert(0.0) += 1.0;
    }

    // Divide the term count by the total number of terms to get the freq
    for count in term_counts.values_mut() {
        *count /= total_terms;
    }

    term_counts
}
