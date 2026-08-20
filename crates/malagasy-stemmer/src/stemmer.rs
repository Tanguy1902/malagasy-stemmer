use serde::{Deserialize, Serialize};
use crate::dictionary::{default_dictionary, FstDictionary};
use crate::disambiguation::ViterbiScorer;
use crate::morphology::MorphologyEngine;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StemResult {
    pub original: String,
    pub root: String,
    pub confidence: f64,
    pub operation: String,
    pub in_dictionary: bool,
}

#[derive(Default, Clone)]
pub struct MalagasyStemmer;

impl MalagasyStemmer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Extrait la racine (*fototeny*) d'un mot malgache.
    
    pub fn stem(&self, word: &str) -> String {
        self.stem_with_details(word).root
    }

    pub fn stem_with_details(&self, word: &str) -> StemResult {
        let dict = default_dictionary();
        self.stem_with_dict(word, dict)
    }

    pub fn stem_with_dict(&self, word: &str, dict: &FstDictionary) -> StemResult {
        let trimmed = word.trim();
        if trimmed.is_empty() {
            return StemResult {
                original: word.to_string(),
                root: String::new(),
                confidence: 0.0,
                operation: "empty".to_string(),
                in_dictionary: false,
            };
        }

        let candidates = MorphologyEngine::analyze(trimmed, dict);
        let best = ViterbiScorer::select_best_for_word(trimmed, candidates, dict);

        StemResult {
            original: word.to_string(),
            root: best.root,
            confidence: best.score,
            operation: best.operation.to_string(),
            in_dictionary: best.in_dictionary,
        }
    }

    pub fn stem_batch(&self, words: &[&str]) -> Vec<String> {
        let dict = default_dictionary();
        words
            .iter()
            .map(|&w| self.stem_with_dict(w, dict).root)
            .collect()
    }

    pub fn stem_batch_with_details(&self, words: &[&str]) -> Vec<StemResult> {
        let dict = default_dictionary();
        words
            .iter()
            .map(|&w| self.stem_with_dict(w, dict))
            .collect()
    }
}

/// Extrait la racine d'un mot malgache.
///
/// ```
/// use malagasy_stemmer::stem;
/// assert_eq!(stem("mamaky"), "vaky");
/// ```
#[inline]
pub fn stem(word: &str) -> String {
    let stemmer = MalagasyStemmer::new();
    stemmer.stem(word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_stemming_samples() {
        let stemmer = MalagasyStemmer::new();

        // Verbes actifs au présent / passé / futur
        assert_eq!(stemmer.stem("manoratra"), "soratra");
        assert_eq!(stemmer.stem("nanoratra"), "soratra");
        assert_eq!(stemmer.stem("hanoratra"), "soratra");

        // Passif & nominalisations
        assert_eq!(stemmer.stem("soratana"), "soratra");
        assert_eq!(stemmer.stem("mpanoratra"), "soratra");
        assert_eq!(stemmer.stem("fanoratana"), "soratra");

        // Labiales (mamaky, mamboly)
        assert_eq!(stemmer.stem("mamaky"), "vaky");
        assert_eq!(stemmer.stem("vakina"), "vaky");
        assert_eq!(stemmer.stem("mamboly"), "voly");

        // Mi- verbes
        assert_eq!(stemmer.stem("mianatra"), "anatra");
        assert_eq!(stemmer.stem("fampianarana"), "anatra");
        assert_eq!(stemmer.stem("mpianatra"), "anatra");

        // Réduplication & composés
        assert_eq!(stemmer.stem("moramora"), "mora");
        assert_eq!(stemmer.stem("tsaratsara"), "tsara");
        assert_eq!(stemmer.stem("tanan-dehibe"), "tanana_lehibe");
    }
}
