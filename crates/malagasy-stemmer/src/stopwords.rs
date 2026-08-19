use std::collections::HashSet;
use std::sync::OnceLock;

static STOPWORDS: OnceLock<HashSet<&'static str>> = OnceLock::new();

const STOPWORD_LIST: &[&str] = &[
    "aho", "izaho", "ianao", "izy", "isika", "izahay", "ianareo", "izy ireo",
    "ny", "ilay", "ity", "io", "iry", "ireo", "ireto", "iretsy",
    "amin", "amin'ny", "tamin", "tamin'ny", "an'ny", "an'i", "amin'i", "tamin'i", "ao", "eo", "eto", "any", "eny",
    "ho", "ho an'ny", "momba", "noho", "amin'izany",
    "sy", "ary", "fa", "nefa", "kanefa", "satria", "raha", "rehefa",
    "mba", "ka", "dia", "izay", "koa", "no",
    "tsy", "efa", "mbola", "vao", "avy",
    "ity", "io", "iry", "izany", "izao", "toy", "toy izany",
    "ve", "angamba", "tokoa", "mihitsy", "kely", "be", "indrindra",
    "dia", "no", "moa",
    "rehetra", "maro", "vitsivitsy", "sasany", "tsirairay",
    "ko", "nao", "ny", "ntsika", "nareo",
    "iray", "roa", "telo", "efatra", "dimy",
];

fn get_stopwords() -> &'static HashSet<&'static str> {
    STOPWORDS.get_or_init(|| {
        let mut set = HashSet::with_capacity(STOPWORD_LIST.len());
        for &word in STOPWORD_LIST {
            set.insert(word);
        }
        set
    })
}

/// Vérifie si un mot est un stopword malgache.

#[inline]
pub fn is_stopword(word: &str) -> bool {
    get_stopwords().contains(word)
}

pub fn count() -> usize {
    get_stopwords().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_stopwords() {
        assert!(is_stopword("ny"));
        assert!(is_stopword("dia"));
        assert!(is_stopword("sy"));
        assert!(is_stopword("aho"));
        assert!(is_stopword("tsy"));
    }

    #[test]
    fn test_content_words_are_not_stopwords() {
        assert!(!is_stopword("soratra"));
        assert!(!is_stopword("trano"));
        assert!(!is_stopword("firenena"));
    }

    #[test]
    fn test_stopword_count() {
        assert!(count() > 50, "Au moins 50 stopwords attendus");
    }
}
