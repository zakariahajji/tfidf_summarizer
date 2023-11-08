use tfidf_summarizer::Document;

// This struct represents a document with an ID and text content.

// This function creates a sample collection of documents.
pub fn create_sample_documents() -> Vec<Document> {
    vec![
        Document {
            id: 1,
            text: "New advancements in renewable energy have seen a significant reduction in costs over the past year.".to_string(),
        },
        Document {
            id: 2,
            text: "Local elections are heating up as candidates prepare their final campaign efforts.".to_string(),
        },
        Document {
            id: 3,
            text: "A breakthrough in biodegradable plastics could see wide-scale adoption in the industry.".to_string(),
        },
        Document {
            id: 4,
            text: "Tech companies are facing increased scrutiny over user data and privacy concerns.".to_string(),
        },
        Document {
            id: 5,
            text: "Healthcare professionals warn about the rise of antibiotic-resistant bacteria.".to_string(),
        },
    ]
}

// Main function for demonstration purposes.
fn main() {
    let documents = create_sample_documents();
    let mut docs_tokens = Vec::new();

    // Tokenize each document
    for doc in &documents {
        let tokens = tfidf_summarizer::tokenize(&doc.text);
        docs_tokens.push(tokens);
    }

    // Calculate document frequencies
    let df = tfidf_summarizer::document_frequency(&docs_tokens);

    // Calculate IDF for the entire corpus
    let idf = tfidf_summarizer::inverse_document_frequency(&df, documents.len());

    // Calculate TF-IDF for each document
    for (idx, tokens) in docs_tokens.iter().enumerate() {
        let tf_idf_scores = tfidf_summarizer::tf_idf(tokens.clone(), &idf);
        println!("Document ID: {}", documents[idx].id);
        for (term, score) in tf_idf_scores {
            println!("Term: {:<15} TF-IDF Score: {}", term, score);
        }
        println!("---------------------------------");
    }
}
