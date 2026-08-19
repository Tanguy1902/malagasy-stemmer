use crate::dictionary::FstDictionary;
use crate::morphology::rules::{ROOT_RESTORATIONS, SUFFIX_RULES};

#[derive(Debug, Clone)]
pub struct SuffixCandidate {
    pub root: String,
    pub suffix: String,
    pub restoration: String,
    pub weight: f64,
}

pub fn strip_suffixes(word: &str, dict: &FstDictionary) -> Vec<SuffixCandidate> {
    let mut candidates = Vec::new();

    for rule in SUFFIX_RULES {
        if let Some(base) = word.strip_suffix(rule.suffix) {
            if base.len() < 2 {
                continue;
            }

            if dict.contains(base) {
                candidates.push(SuffixCandidate {
                    root: base.to_string(),
                    suffix: rule.suffix.to_string(),
                    restoration: String::new(),
                    weight: rule.weight * 0.70,
                });
            }

            for &restoration in rule.restorations {
                let candidate = format!("{}{}", base, restoration);
                if candidate != word && dict.contains(&candidate) {
                    candidates.push(SuffixCandidate {
                        root: candidate,
                        suffix: rule.suffix.to_string(),
                        restoration: restoration.to_string(),
                        weight: rule.weight * 0.98,
                    });
                }
            }

            if base.ends_with('t') {
                let cand_tra = format!("{}ra", base);
                if dict.contains(&cand_tra) {
                    candidates.push(SuffixCandidate {
                        root: cand_tra,
                        suffix: rule.suffix.to_string(),
                        restoration: "ra".to_string(),
                        weight: rule.weight * 1.0,
                    });
                }
            }

            if base.ends_with('r') {
                let cand_tra = format!("{}tra", &base[..base.len() - 1]);
                if dict.contains(&cand_tra) {
                    candidates.push(SuffixCandidate {
                        root: cand_tra,
                        suffix: rule.suffix.to_string(),
                        restoration: "r->tra".to_string(),
                        weight: rule.weight * 1.0,
                    });
                }
            }

            if base.ends_with('h') {
                let cand_ka = format!("{}ka", &base[..base.len() - 1]);
                if dict.contains(&cand_ka) {
                    candidates.push(SuffixCandidate {
                        root: cand_ka,
                        suffix: rule.suffix.to_string(),
                        restoration: "h->ka".to_string(),
                        weight: rule.weight * 1.0,
                    });
                }
            }

            if base.ends_with("en") && base.len() >= 4 {
                let cand_y = format!("{}y", &base[..base.len() - 2]);
                if dict.contains(&cand_y) {
                    candidates.push(SuffixCandidate {
                        root: cand_y,
                        suffix: rule.suffix.to_string(),
                        restoration: "en->y".to_string(),
                        weight: rule.weight * 0.98,
                    });
                }
            }

            if base.ends_with("ez") && base.len() >= 4 {
                let cand_hy = format!("{}hy", &base[..base.len() - 2]);
                if dict.contains(&cand_hy) {
                    candidates.push(SuffixCandidate {
                        root: cand_hy,
                        suffix: rule.suffix.to_string(),
                        restoration: "ez->hy".to_string(),
                        weight: rule.weight * 0.98,
                    });
                }
            }

            if base.ends_with('i') && base.len() >= 3 {
                let cand_y = format!("{}y", &base[..base.len() - 1]);
                if dict.contains(&cand_y) {
                    candidates.push(SuffixCandidate {
                        root: cand_y,
                        suffix: rule.suffix.to_string(),
                        restoration: "i->y".to_string(),
                        weight: rule.weight * 0.98,
                    });
                }
            }

            if dict.contains(base) {
                candidates.push(SuffixCandidate {
                    root: base.to_string(),
                    suffix: rule.suffix.to_string(),
                    restoration: "direct_base".to_string(),
                    weight: rule.weight * 0.95,
                });
            }
        }
    }

    for &restoration in ROOT_RESTORATIONS {
        if word.len() > restoration.len() + 2 {
            let base_len = word.len().saturating_sub(1);
            let base = &word[..base_len];
            let candidate = format!("{}{}", base, restoration);
            if dict.contains(&candidate) && candidate != word {
                candidates.push(SuffixCandidate {
                    root: candidate,
                    suffix: String::new(),
                    restoration: restoration.to_string(),
                    weight: 0.3,
                });
            }
        }
    }

    candidates
}

pub fn strip_suffix_only(word: &str) -> (&str, Option<&'static str>) {
    for rule in SUFFIX_RULES {
        if let Some(base) = word.strip_suffix(rule.suffix) {
            if base.len() >= 2 {
                return (base, Some(rule.suffix));
            }
        }
    }
    (word, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ina_suffix() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = strip_suffixes("vakina", dict);
        assert!(
            candidates.iter().any(|c| c.root == "vaky"),
            "vakina doit restaurer → vaky"
        );
    }

    #[test]
    fn test_strip_ana_suffix() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = strip_suffixes("soratana", dict);
        assert!(
            candidates.iter().any(|c| c.root == "soratra"),
            "soratana doit restaurer → soratra"
        );
    }

    #[test]
    fn test_strip_ana_suffix_anatra() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = strip_suffixes("anarana", dict);
        assert!(
            candidates.iter().any(|c| c.root == "anatra"),
            "anarana doit restaurer → anatra"
        );
    }

    #[test]
    fn test_strip_suffix_only() {
        let (base, suffix) = strip_suffix_only("vakina");
        assert_eq!(base, "vak");
        assert_eq!(suffix, Some("ina"));
    }

    #[test]
    fn test_vowel_alternation_teny() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = strip_suffixes("tenenina", dict);
        assert!(
            candidates.iter().any(|c| c.root == "teny"),
            "tenenina doit produire la racine 'teny'"
        );
    }

    #[test]
    fn test_vowel_alternation_tsindry() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = strip_suffixes("tsindriana", dict);
        assert!(
            candidates.iter().any(|c| c.root == "tsindry"),
            "tsindriana doit produire la racine 'tsindry'"
        );
    }

    #[test]
    fn test_direct_passive_lazaina() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = strip_suffixes("lazaina", dict);
        assert!(
            candidates.iter().any(|c| c.root == "laza"),
            "lazaina doit produire la racine 'laza'"
        );
    }

    #[test]
    fn test_imperative_soraty_and_tapaho() {
        let dict = crate::dictionary::default_dictionary();
        let cands_soraty = strip_suffixes("soraty", dict);
        assert!(
            cands_soraty.iter().any(|c| c.root == "soratra"),
            "soraty (impératif) doit restaurer 'soratra'"
        );

        let cands_tapaho = strip_suffixes("tapaho", dict);
        assert!(
            cands_tapaho.iter().any(|c| c.root == "tapaka"),
            "tapaho (impératif) doit restaurer 'tapaka'"
        );
    }
}
