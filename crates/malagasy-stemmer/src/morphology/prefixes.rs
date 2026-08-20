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
                for (idx, &restoration) in rule.vowel_restorations.iter().enumerate() {
                    let candidate = format!("{}{}", restoration, remainder);
                    let in_dict = dict.contains(&candidate);
                    candidates.push(PrefixCandidate {
                        root: candidate.clone(),
                        prefix: rule.prefix.to_string(),
                        weight: if in_dict { 1.0 } else { 0.85 - (idx as f64 * 0.01) },
                    });

                    if let Some(redup) = crate::morphology::reduplication::strip_reduplication(&candidate) {
                        if dict.contains(&redup.root) {
                            candidates.push(PrefixCandidate {
                                root: redup.root,
                                prefix: format!("{}+redup", rule.prefix),
                                weight: 0.99,
                            });
                        }
                    }
                }
                let is_coronal_nasal = rule.prefix.ends_with('n');
                if is_coronal_nasal {
                    let as_is = remainder.to_string();
                    let in_dict = dict.contains(&as_is);
                    let weight = if in_dict {
                        if as_is.len() >= 4 { 1.0 } else { 0.85 }
                    } else {
                        0.50
                    };
                    candidates.push(PrefixCandidate {
                        root: as_is,
                        prefix: rule.prefix.to_string(),
                        weight,
                    });
                }
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
                        weight: 0.95,
                    });
                }
            }
        }
    }

    for &prefix in SIMPLE_PREFIXES {
        if let Some(remainder) = word.strip_prefix(prefix) {
            if remainder.len() >= 2 {
                let in_dict = dict.contains(remainder);

                let is_nasal_prefixed = (prefix == "fa" && (word.starts_with("fam") || word.starts_with("fan") || word.starts_with("fang")))
                    || (prefix == "ma" && (word.starts_with("mam") || word.starts_with("man") || word.starts_with("mang")))
                    || (prefix == "ha" && (word.starts_with("ham") || word.starts_with("han") || word.starts_with("hang")))
                    || (prefix == "na" && (word.starts_with("nam") || word.starts_with("nan") || word.starts_with("nang")))
                    || (prefix == "mp" && (word.starts_with("mpam") || word.starts_with("mpan") || word.starts_with("mpang")))
                    || (prefix == "fi" && (word.starts_with("famp") || word.starts_with("fampi") || word.starts_with("fampif")));

                if is_nasal_prefixed && !in_dict {
                    continue;
                }
                let is_weak_short_prefix = (prefix == "ma" || prefix == "fa" || prefix == "ha" || prefix == "fi")
                    && (word.ends_with("ina") || word.ends_with("io") || word.ends_with("ena"));
                let is_causative_prefix = prefix == "fana" || prefix == "mana" || prefix == "nana" || prefix == "hana" || prefix == "mpana";
                let is_single_vowel_prefix = prefix == "a" || prefix == "i";

                let base_w = if in_dict {
                    if is_weak_short_prefix {
                        0.40
                    } else if is_single_vowel_prefix {
                        0.92
                    } else if is_causative_prefix {
                        1.0
                    } else if is_nasal_prefixed {
                        0.95
                    } else {
                        1.0
                    }
                } else {
                    if is_weak_short_prefix { 0.20 } else { 0.50 }
                };

                candidates.push(PrefixCandidate {
                    root: remainder.to_string(),
                    prefix: prefix.to_string(),
                    weight: base_w,
                });

                if let Some(redup) = crate::morphology::reduplication::strip_reduplication(remainder) {
                    if dict.contains(&redup.root) {
                        candidates.push(PrefixCandidate {
                            root: redup.root,
                            prefix: format!("{}+redup", prefix),
                            weight: 0.98,
                        });
                    }
                }

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
