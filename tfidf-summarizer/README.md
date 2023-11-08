# TF-IDF Summarizer

`tfidf-summarizer` is a Rust crate for computing Term Frequency-Inverse Document Frequency (TF-IDF) vectors for a set of documents. This statistical measure is used to evaluate how important a word is to a document in a collection or corpus.

## Features

- Tokenization of text data.
- Calculation of term frequency (TF) for words in each document.
- Calculation of document frequency (DF) for words across all documents.
- Calculation of inverse document frequency (IDF) across all documents.
- Computation of TF-IDF scores for words in each document.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tfidf-summarizer = "0.1.0"
