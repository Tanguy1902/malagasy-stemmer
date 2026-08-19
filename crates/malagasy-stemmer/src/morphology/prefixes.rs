use crate::dictionary::FstDictionary;
use crate::morphology::rules::{NASAL_MUTATIONS, SIMPLE_PREFIXES};

#[derive(Debug, Clone)]
pub struct PrefixCandidate {
    pub root: String,
    pub prefix: String,
    pub weight: f64,
}

pub fn generate_prefix_candidates(word: &str, dict: &FstDictionary) -> Vec<PrefixCandidate> {
    let mut candidates = Vec::new();

    for rule in NASAL_MUTATIONS {
        if let Some(remainder) = word.strip_prefix(rule.prefix) {
            if remainder.is_empty() {
                continue;
            }

            let first_char = remainder.chars().next().unwrap();

            if is_vowel(first_char) {
                for &restoration in rule.vowel_restorations {
                    let candidate = format!("{}{}", restoration, remainder);
                    let in_dict = dict.contains(&candidate);
                    candidates.push(PrefixCandidate {
                        root: candidate,
                        prefix: rule.prefix.to_string(),
                        weight: if in_dict { 1.0 } else { 0.85 },
                    });
                }
                let as_is = remainder.to_string();
                let in_dict = dict.contains(&as_is);
                candidates.push(PrefixCandidate {
                    root: as_is,
                    prefix: rule.prefix.to_string(),
                    weight: if in_dict { 1.0 } else { 0.85 },
                });
            }

            for &(surface, lexical) in rule.consonant_mutations {
                if remainder.starts_with(surface) {
                    let candidate = format!("{}{}", lexical, &remainder[surface.len()..]);
                    let in_dict = dict.contains(&candidate);
                    candidates.push(PrefixCandidate {
                        root: candidate,
                        prefix: rule.prefix.to_string(),
                        weight: if in_dict { 1.0 } else { 0.88 },
                    });
                }
            }

            if !is_vowel(first_char) {
                let direct = remainder.to_string();
                if dict.contains(&direct) {
                    candidates.push(PrefixCandidate {
                        root: direct,
                        prefix: rule.prefix.to_string(),
                        weight: 0.9,
                    });
                }
            }
        }
    }

    for &prefix in SIMPLE_PREFIXES {
        if let Some(remainder) = word.strip_prefix(prefix) {
            if remainder.len() >= 2 {
                let is_weak_short_prefix = (prefix == "ma" || prefix == "fa" || prefix == "ha" || prefix == "fi")
                    && (word.starts_with("mam") || word.starts_with("man") || word.starts_with("mang")
                        || word.starts_with("fam") || word.starts_with("fan") || word.starts_with("fang")
                        || word.starts_with("ham") || word.starts_with("han") || word.starts_with("hang")
                        || (prefix == "fi" && (word.ends_with("ina") || word.ends_with("io") || word.ends_with("ena"))));

                let in_dict = dict.contains(remainder);
                let base_w = if in_dict {
                    if is_weak_short_prefix { 0.40 } else { 1.0 }
                } else {
                    if is_weak_short_prefix { 0.20 } else { 0.50 }
                };

                candidates.push(PrefixCandidate {
                    root: remainder.to_string(),
                    prefix: prefix.to_string(),
                    weight: base_w,
                });

                if remainder.starts_with('i') && remainder.len() > 3 {
                    let sub_rest = &remainder[1..];
                    if dict.contains(sub_rest) {
                        candidates.push(PrefixCandidate {
                            root: sub_rest.to_string(),
                            prefix: format!("{}+i", prefix),
                            weight: 0.95,
                        });
                    }
                }
            }
        }
    }

    candidates
}

#[inline]
fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'y')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_man_prefix_soratra() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = generate_prefix_candidates("manoratra", dict);
        assert!(
            candidates.iter().any(|c| c.root == "soratra"),
            "manoratra doit produire le candidat 'soratra'"
        );
    }

    #[test]
    fn test_mam_prefix_vaky() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = generate_prefix_candidates("mamaky", dict);
        assert!(
            candidates.iter().any(|c| c.root == "vaky"),
            "mamaky doit produire le candidat 'vaky'"
        );
    }

    #[test]
    fn test_nan_prefix_past_tense() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = generate_prefix_candidates("nanoratra", dict);
        assert!(
            candidates.iter().any(|c| c.root == "soratra"),
            "nanoratra (passé) doit produire 'soratra'"
        );
    }

    #[test]
    fn test_mi_simple_prefix() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = generate_prefix_candidates("mianatra", dict);
        assert!(
            candidates.iter().any(|c| c.root == "anatra"),
            "mianatra doit produire 'anatra'"
        );
    }

    #[test]
    fn test_mpan_agent_prefix() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = generate_prefix_candidates("mpanoratra", dict);
        assert!(
            candidates.iter().any(|c| c.root == "soratra"),
            "mpanoratra doit produire 'soratra'"
        );
    }
}
