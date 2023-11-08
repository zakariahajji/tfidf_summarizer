use unicode_segmentation::UnicodeSegmentation;

/// Tokenizes the given text into a vector of lowercase alphanumeric words.

pub fn tokenize(text: &str) -> Vec<String> {
    // split the text into words using Unicode segmentation
    let words = text.unicode_words().collect::<Vec<&str>>();

    // process each word to handle punctuation and case
    let tokens = words
        .into_iter()
        .map(|word| {
            word.trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
        })
        .filter(|token| !token.is_empty())
        .collect::<Vec<String>>();

    tokens
}
