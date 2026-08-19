use crate::dictionary::{default_dictionary, FstDictionary, FuzzyMatch};

/// Recherche les racines les plus proches dans le dictionnaire pour un mot donné.
///
/// ```
/// use malagasy_stemmer::fuzzy::fuzzy_root_lookup;
/// let matches = fuzzy_root_lookup("sorata", 1);
/// assert!(matches.iter().any(|m| m.word == "soratra"));
/// ```
pub fn fuzzy_root_lookup(word: &str, max_distance: u32) -> Vec<FuzzyMatch> {
    let dict = default_dictionary();
    fuzzy_root_lookup_with_dict(word, max_distance, dict)
}

pub fn fuzzy_root_lookup_with_dict(
    word: &str,
    max_distance: u32,
    dict: &FstDictionary,
) -> Vec<FuzzyMatch> {
    dict.fuzzy_search(word, max_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_lookup_single_typo() {
        let matches = fuzzy_root_lookup("sorata", 1);
        assert!(!matches.is_empty());
        assert_eq!(matches[0].word, "soratra");
        assert_eq!(matches[0].distance, 1);
    }

    #[test]
    fn test_fuzzy_lookup_exact_word() {
        let matches = fuzzy_root_lookup("tsara", 1);
        assert!(!matches.is_empty());
        assert_eq!(matches[0].word, "tsara");
        assert_eq!(matches[0].distance, 0);
    }
}
