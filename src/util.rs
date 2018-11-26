use std::collections::HashMap;
use std::f32::EPSILON;

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

pub fn cos_similarity(x: &[usize], y: &[usize]) -> f32 {
    let x0 = x.iter().map(|&x| (x * x) as f32).sum::<f32>().sqrt() + EPSILON;
    let y0 = y.iter().map(|&y| (y * y) as f32).sum::<f32>().sqrt() + EPSILON;
    x.iter()
        .zip(y.iter())
        .map(|(&x, &y)| (x as f32 / x0) * (y as f32 / y0))
        .sum()
}

pub fn most_similar<'a>(
    query: &str,
    word_to_id: &WordToId,
    id_to_word: &'a IdToWord,
    word_matrix: &CoMatrix,
) -> Vec<(&'a str, f32)> {
    // (1)
    let query_id = if let Some(id) = word_to_id.0.get(query) {
        id
    } else {
        return Vec::new();
    };

    let query_vec = &word_matrix.0[query_id.0];

    // (2)
    let mut similarity = word_matrix
        .0
        .iter()
        .enumerate()
        .filter(|t| query_id.0 != t.0)
        .map(|(i, v)| {
            (
                id_to_word.0[&WordId(i)].as_str(),
                cos_similarity(v, query_vec),
            )
        }).collect::<Vec<_>>();
    similarity.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    similarity.reverse();
    similarity
}

#[derive(Debug, Clone, Default)]
pub struct PmiMatrix(pub Vec<Vec<f32>>);

pub fn ppmi(c: &CoMatrix) -> PmiMatrix {
    let mut m = vec![vec![0.0; c.0[0].len()]; c.0.len()];

    let n = c.0.iter().map(|t| t.iter().sum::<usize>()).sum::<usize>();
    let mut s = vec![0; c.0[0].len()];
    for i in 0..s.len() {
        for j in 0..c.0.len() {
            s[i] += c.0[j][i];
        }
    }

    for i in 0..c.0.len() {
        for j in 0..c.0[0].len() {
            let pmi = ((((c.0[i][j] * n) as f32) / (s[j] * s[i]) as f32) + EPSILON).log2();
            if pmi > 0.0 {
                m[i][j] = pmi;
            }
        }
    }
    PmiMatrix(m)
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

    #[test]
    fn cos_similarity_works() {
        let text = "You say goodbye and I say hello.";
        let (corpus, word_to_id, _id_to_word) = prerocess(text);
        let c = create_co_matrix(&corpus, 1);
        assert_eq!(
            cos_similarity(&c.0[word_to_id.0["you"].0], &c.0[word_to_id.0["i"].0]),
            0.70710665
        );
    }

    #[test]
    fn most_similar_works() {
        let text = "You say goodbye and I say hello.";
        let (corpus, word_to_id, id_to_word) = prerocess(text);
        let c = create_co_matrix(&corpus, 1);
        let similarity = most_similar("you", &word_to_id, &id_to_word, &c);
        assert_eq!(
            &similarity[..5],
            [
                ("hello", 0.70710665),
                ("i", 0.70710665),
                ("goodbye", 0.70710665),
                (".", 0.0),
                ("and", 0.0)
            ]
        );
    }
}
