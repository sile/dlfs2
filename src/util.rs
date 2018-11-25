use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WordId(pub usize);

#[derive(Debug, Clone, Default)]
pub struct Corpus(pub Vec<WordId>);

#[derive(Debug, Clone, Default)]
pub struct WordToId(pub HashMap<String, WordId>);

#[derive(Debug, Clone, Default)]
pub struct IdToWord(pub HashMap<WordId, String>);

pub fn prerocess(text: &str) -> (Corpus, WordToId, IdToWord) {
    let text = text.to_ascii_lowercase().replace('.', " .");
    let words = text.split(' ');
    let mut word_to_id = WordToId::default();
    let mut id_to_word = IdToWord::default();
    let mut corpus = Corpus::default();
    for word in words {
        if !word_to_id.0.contains_key(word) {
            let new_id = WordId(word_to_id.0.len());
            word_to_id.0.insert(word.to_owned(), new_id);
            id_to_word.0.insert(new_id, word.to_owned());
        }
        corpus.0.push(word_to_id.0[word]);
    }
    (corpus, word_to_id, id_to_word)
}

#[derive(Debug, Clone, Default)]
pub struct CoMatrix(pub Vec<Vec<usize>>);

pub fn create_co_matrix(corpus: &Corpus, window_size: usize) -> CoMatrix {
    let vocab_size = corpus.0.iter().max().map_or(0, |w| w.0 + 1);
    let mut m = vec![vec![0; vocab_size]; vocab_size];
    for (idx, word_id) in corpus.0.iter().enumerate() {
        for i in 1..window_size + 1 {
            if let Some(left_idx) = idx.checked_sub(i) {
                let left_word_id = corpus.0[left_idx];
                m[word_id.0][left_word_id.0] += 1;
            }

            let right_idx = idx + i;
            if right_idx < corpus.0.len() {
                let right_word_id = corpus.0[right_idx];
                m[word_id.0][right_word_id.0] += 1;
            }
        }
    }
    CoMatrix(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prerocess_works() {
        let text = "You say goodbye and I say hello.";
        let (corpus, _word_to_id, _id_to_word) = prerocess(text);
        assert_eq!(
            corpus.0.iter().map(|w| w.0).collect::<Vec<_>>(),
            [0, 1, 2, 3, 4, 1, 5, 6]
        );
    }

    #[test]
    fn create_co_matrix_works() {
        let text = "You say goodbye and I say hello.";
        let (corpus, word_to_id, _id_to_word) = prerocess(text);
        let c = create_co_matrix(&corpus, 1);
        assert_eq!(c.0[word_to_id.0["goodbye"].0], [0, 1, 0, 1, 0, 0, 0]);
    }
}
