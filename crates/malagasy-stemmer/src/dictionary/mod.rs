use std::sync::OnceLock;

use fst::automaton::Levenshtein;
use fst::{IntoStreamer, Set, Streamer};

static FST_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/roots.fst"));
static DICTIONARY: OnceLock<FstDictionary> = OnceLock::new();

pub struct FstDictionary {
    set: Set<Vec<u8>>,
}

impl FstDictionary {
    pub fn from_embedded() -> Self {
        let set = Set::new(FST_DATA.to_vec())
            .expect("Les données FST embarquées sont invalides");
        FstDictionary { set }
    }

    pub fn from_sorted_roots(roots: &[&str]) -> Self {
        let mut builder = fst::SetBuilder::memory();
        for root in roots {
            builder.insert(root.as_bytes()).unwrap_or_else(|e| {
                panic!("Erreur insertion '{}': {}", root, e);
            });
        }
        let bytes = builder.into_inner().expect("Erreur construction FST");
        let set = Set::new(bytes).expect("FST invalide");
        FstDictionary { set }
    }

    #[inline]
    pub fn contains(&self, word: &str) -> bool {
        self.set.contains(word.as_bytes())
    }

    pub fn fuzzy_search(&self, word: &str, max_distance: u32) -> Vec<FuzzyMatch> {
        let Ok(automaton) = Levenshtein::new(word, max_distance) else {
            return Vec::new();
        };

        let mut results = Vec::new();
        let mut stream = self.set.search(&automaton).into_stream();
        while let Some(key) = stream.next() {
            if let Ok(matched_word) = std::str::from_utf8(key) {
                let distance = levenshtein_distance(word, matched_word);
                results.push(FuzzyMatch {
                    word: matched_word.to_string(),
                    distance,
                });
            }
        }
        results.sort_by_key(|m| m.distance);
        results
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct FuzzyMatch {
    pub word: String,
    pub distance: u32,
}

pub fn default_dictionary() -> &'static FstDictionary {
    DICTIONARY.get_or_init(FstDictionary::from_embedded)
}

fn levenshtein_distance(a: &str, b: &str) -> u32 {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let n = a_chars.len();
    let m = b_chars.len();

    if n == 0 { return m as u32; }
    if m == 0 { return n as u32; }

    let mut prev = vec![0u32; m + 1];
    let mut curr = vec![0u32; m + 1];

    for j in 0..=m {
        prev[j] = j as u32;
    }

    for i in 1..=n {
        curr[0] = i as u32;
        for j in 1..=m {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1)
                .min(curr[j - 1] + 1)
                .min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_dictionary_loads() {
        let dict = default_dictionary();
        assert!(!dict.is_empty(), "Le dictionnaire embarqué ne doit pas être vide");
        assert!(dict.len() > 200, "Le dictionnaire doit contenir au moins 200 racines");
    }

    #[test]
    fn test_contains_common_roots() {
        let dict = default_dictionary();
        assert!(dict.contains("soratra"));
        assert!(dict.contains("vaky"));
        assert!(dict.contains("tsara"));
        assert!(dict.contains("anatra"));
        assert!(dict.contains("trano"));
        assert!(!dict.contains("manoratra")); // Forme dérivée, pas une racine
        assert!(!dict.contains("zxywq")); // Mot inexistant
    }

    #[test]
    fn test_fuzzy_search() {
        let dict = default_dictionary();
        let results = dict.fuzzy_search("sorata", 1); // Manque le 'r'
        assert!(!results.is_empty(), "La recherche floue doit trouver 'soratra'");
        assert!(results.iter().any(|m| m.word == "soratra"));
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("soratra", "sorata"), 1);
    }

    #[test]
    fn test_from_sorted_roots() {
        let roots = &["anatra", "soratra", "trano", "tsara", "vaky"];
        let dict = FstDictionary::from_sorted_roots(roots);
        assert_eq!(dict.len(), 5);
        assert!(dict.contains("soratra"));
        assert!(!dict.contains("mora"));
    }
}
