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
}
