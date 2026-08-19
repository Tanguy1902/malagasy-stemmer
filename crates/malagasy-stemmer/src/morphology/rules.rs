#[derive(Debug, Clone)]
pub struct NasalMutationRule {
    pub prefix: &'static str,
    pub vowel_restorations: &'static [&'static str],
    pub consonant_mutations: &'static [(&'static str, &'static str)],
}

pub static NASAL_MUTATIONS: &[NasalMutationRule] = &[
    NasalMutationRule {
        prefix: "man",
        vowel_restorations: &["s", "t", "ts", "h"],
        consonant_mutations: &[
            ("dr", "r"),
            ("d", "l"),
            ("d", "t"),
            ("d", "d"),
            ("j", "z"),
            ("j", "j"),
        ],
    },
    NasalMutationRule {
        prefix: "mam",
        vowel_restorations: &["v", "b", "p", "f"],
        consonant_mutations: &[
            ("b", "v"),
            ("b", "b"),
            ("p", "f"),
        ],
    },
    NasalMutationRule {
        prefix: "mang",
        vowel_restorations: &["h", "k", "g"],
        consonant_mutations: &[],
    },
    NasalMutationRule {
        prefix: "nan",
        vowel_restorations: &["s", "t", "ts", "h"],
        consonant_mutations: &[
            ("dr", "r"),
            ("d", "l"),
            ("d", "t"),
            ("d", "d"),
            ("j", "z"),
            ("j", "j"),
        ],
    },
    NasalMutationRule {
        prefix: "nam",
        vowel_restorations: &["v", "b", "p", "f"],
        consonant_mutations: &[("b", "v"), ("b", "b"), ("p", "f")],
    },
    NasalMutationRule {
        prefix: "nang",
        vowel_restorations: &["h", "k", "g"],
        consonant_mutations: &[],
    },
    NasalMutationRule {
        prefix: "han",
        vowel_restorations: &["s", "t", "ts", "h"],
        consonant_mutations: &[
            ("dr", "r"),
            ("d", "l"),
            ("d", "t"),
            ("d", "d"),
            ("j", "z"),
            ("j", "j"),
        ],
    },
    NasalMutationRule {
        prefix: "ham",
        vowel_restorations: &["v", "b", "p", "f"],
        consonant_mutations: &[("b", "v"), ("b", "b"), ("p", "f")],
    },
    NasalMutationRule {
        prefix: "hang",
        vowel_restorations: &["h", "k", "g"],
        consonant_mutations: &[],
    },
    NasalMutationRule {
        prefix: "fan",
        vowel_restorations: &["s", "t", "ts", "h"],
        consonant_mutations: &[
            ("dr", "r"),
            ("d", "l"),
            ("d", "t"),
            ("d", "d"),
            ("j", "z"),
            ("j", "j"),
        ],
    },
    NasalMutationRule {
        prefix: "fam",
        vowel_restorations: &["v", "b", "p", "f"],
        consonant_mutations: &[("b", "v"), ("b", "b"), ("p", "f")],
    },
    NasalMutationRule {
        prefix: "fang",
        vowel_restorations: &["h", "k", "g"],
        consonant_mutations: &[],
    },
    NasalMutationRule {
        prefix: "mpan",
        vowel_restorations: &["s", "t", "ts", "h"],
        consonant_mutations: &[
            ("dr", "r"),
            ("d", "l"),
            ("d", "t"),
            ("d", "d"),
            ("j", "z"),
            ("j", "j"),
        ],
    },
    NasalMutationRule {
        prefix: "mpam",
        vowel_restorations: &["v", "b", "p", "f"],
        consonant_mutations: &[("b", "v"), ("b", "b"), ("p", "f")],
    },
    NasalMutationRule {
        prefix: "mpang",
        vowel_restorations: &["h", "k", "g"],
        consonant_mutations: &[],
    },
];

pub static SIMPLE_PREFIXES: &[&str] = &[
    "mpampif",
    "fampif",
    "mampif",
    "nampif",
    "hampif",
    "mpampi",
    "fampi",
    "mampi",
    "nampi",
    "hampi",
    "mpamaha",
    "famaha",
    "mamaha",
    "namaha",
    "hamaha",
    "mpamp",
    "famp",
    "mamp",
    "namp",
    "hamp",
    "mpanka",
    "fanka",
    "manka",
    "nanka",
    "hanka",
    "mpana",
    "fana",
    "mana",
    "nana",
    "hana",
    "maha",
    "mpaha",
    "faha",
    "tafa",
    "mif",
    "nif",
    "hif",
    "fif",
    "mpi",
    "mi",
    "ni",
    "hi",
    "fi",
    "amp",
    "if",
    "mp",
    "fa",
    "ha",
    "ma",
    "na",
    "i",
    "a",
];

#[derive(Debug, Clone)]
pub struct SuffixRule {
    pub suffix: &'static str,
    pub restorations: &'static [&'static str],
    pub weight: f64,
}

pub static SUFFIX_RULES: &[SuffixRule] = &[
    SuffixRule { suffix: "antsika", restorations: &["a", "y", "na", "tra", "ka", "ra"], weight: 0.7 },
    SuffixRule { suffix: "anareo", restorations: &["a", "y", "na", "tra", "ka", "ra"], weight: 0.7 },
    SuffixRule { suffix: "ntsika", restorations: &["a", "y", "na", "tra"], weight: 0.7 },
    SuffixRule { suffix: "anare", restorations: &["a", "y", "na", "tra", "ka", "ra"], weight: 0.7 },
    SuffixRule { suffix: "tsika", restorations: &["a", "y", "na", "tra"], weight: 0.7 },
    SuffixRule { suffix: "nareo", restorations: &["a", "y", "na", "tra"], weight: 0.7 },
    SuffixRule { suffix: "anay", restorations: &["a", "y", "na", "tra", "ka", "ra"], weight: 0.7 },
    SuffixRule { suffix: "areo", restorations: &["a", "y", "na", "tra"], weight: 0.7 },
    SuffixRule { suffix: "zina", restorations: &["y", "a", "o"], weight: 0.95 },
    SuffixRule { suffix: "sina", restorations: &["y", "a", "o"], weight: 0.95 },
    SuffixRule { suffix: "vina", restorations: &["y", "a", "o"], weight: 0.95 },
    SuffixRule { suffix: "hina", restorations: &["ka", "y", "a"], weight: 0.95 },
    SuffixRule { suffix: "tina", restorations: &["tra", "y", "a"], weight: 0.95 },
    SuffixRule { suffix: "rina", restorations: &["tra", "ra", "y"], weight: 0.95 },
    SuffixRule { suffix: "nina", restorations: &["na", "y", "a"], weight: 0.95 },
    SuffixRule { suffix: "fina", restorations: &["y", "a"], weight: 0.95 },
    SuffixRule { suffix: "ina", restorations: &["", "y", "a", "na", "tra", "ka", "ra", "e"], weight: 1.0 },
    SuffixRule { suffix: "ana", restorations: &["", "a", "y", "na", "tra", "ka", "ra", "e", "o"], weight: 1.0 },
    SuffixRule { suffix: "ena", restorations: &["", "y", "a", "na", "tra", "ka", "ra", "e"], weight: 1.0 },
    SuffixRule { suffix: "ona", restorations: &["", "y", "a", "o", "na", "tra", "ka"], weight: 1.0 },
    SuffixRule { suffix: "nao", restorations: &["", "a", "na", "tra"], weight: 0.6 },
    SuffixRule { suffix: "avy", restorations: &["", "a", "o"], weight: 0.8 },
    SuffixRule { suffix: "ovy", restorations: &["", "a", "o", "y"], weight: 0.8 },
    SuffixRule { suffix: "ny", restorations: &["", "a", "na", "tra"], weight: 0.6 },
    SuffixRule { suffix: "ko", restorations: &["", "a", "na", "tra"], weight: 0.6 },
    SuffixRule { suffix: "na", restorations: &["", "a", "y", "tra", "ka"], weight: 0.6 },
    SuffixRule { suffix: "io", restorations: &["", "y", "a", "tra", "ka"], weight: 0.95 },
    SuffixRule { suffix: "ao", restorations: &["", "a", "y", "tra", "ka"], weight: 0.95 },
    SuffixRule { suffix: "eo", restorations: &["", "y", "a", "tra"], weight: 0.95 },
    SuffixRule { suffix: "o", restorations: &["y", "a", "na", "tra", "ka"], weight: 0.95 },
    SuffixRule { suffix: "y", restorations: &["tra", "ka", "na", "a"], weight: 0.95 },
];

#[derive(Debug, Clone)]
pub struct InfixRule {
    pub infix: &'static str,
    pub description: &'static str,
    pub weight: f64,
}

pub static INFIX_RULES: &[InfixRule] = &[
    InfixRule { infix: "in", description: "Passif / perfectif", weight: 0.8 },
    InfixRule { infix: "om", description: "Potentiel / statif", weight: 0.7 },
];

#[derive(Debug, Clone)]
pub struct SandhiRule {
    pub surface: &'static str,
    pub lexical: &'static str,
}

pub static SANDHI_MUTATIONS: &[SandhiRule] = &[
    SandhiRule { surface: "ts", lexical: "s" },
    SandhiRule { surface: "dr", lexical: "r" },
    SandhiRule { surface: "tr", lexical: "r" },
    SandhiRule { surface: "p", lexical: "f" },
    SandhiRule { surface: "d", lexical: "l" },
    SandhiRule { surface: "b", lexical: "v" },
    SandhiRule { surface: "g", lexical: "k" },
    SandhiRule { surface: "j", lexical: "z" },
];

pub static ROOT_RESTORATIONS: &[&str] = &[
    "tra", "ka", "na", "ra", "y", "a", "e", "o",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nasal_mutations_cover_all_tenses() {
        let prefixes: Vec<&str> = NASAL_MUTATIONS.iter().map(|r| r.prefix).collect();
        assert!(prefixes.contains(&"man"));
        assert!(prefixes.contains(&"nan"));
        assert!(prefixes.contains(&"han"));
        assert!(prefixes.contains(&"fan"));
        assert!(prefixes.contains(&"mpan"));
    }

    #[test]
    fn test_suffix_rules_sorted_by_length() {
        for window in SUFFIX_RULES.windows(2) {
            assert!(
                window[0].suffix.len() >= window[1].suffix.len(),
                "Suffixes non triés : '{}' (len {}) avant '{}' (len {})",
                window[0].suffix,
                window[0].suffix.len(),
                window[1].suffix,
                window[1].suffix.len()
            );
        }
    }
}
