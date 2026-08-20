pub mod circumfixes;
pub mod compounds;
pub mod infixes;
pub mod irregular;
pub mod prefixes;
pub mod reduplication;
pub mod rules;
pub mod suffixes;

use crate::dictionary::FstDictionary;
use circumfixes::strip_circumfixes;
use compounds::stem_compound;
use infixes::strip_infixes;
use irregular::lookup_irregular;
use prefixes::generate_prefix_candidates;
use reduplication::strip_reduplication;
use suffixes::strip_suffixes;

#[derive(Debug, Clone)]
pub struct RawCandidate {
    pub root: String,
    pub operation: &'static str,
    pub base_weight: f64,
}

pub struct MorphologyEngine;

impl MorphologyEngine {
    pub fn analyze(word: &str, dict: &FstDictionary) -> Vec<RawCandidate> {
        let clean_word = word.trim().to_lowercase();
        if clean_word.is_empty() {
            return Vec::new();
        }

        if let Some(irreg_root) = lookup_irregular(&clean_word) {
            return vec![RawCandidate {
                root: irreg_root.to_string(),
                operation: "irregular_suppletive",
                base_weight: 1.0,
            }];
        }

        if let Some(compound) = stem_compound(&clean_word, dict) {
            return vec![RawCandidate {
                root: compound.joined,
                operation: "compound_sandhi",
                base_weight: 0.98,
            }];
        }

        if let Some(redup) = strip_reduplication(&clean_word) {
            let in_dict = dict.contains(&redup.root);
            if in_dict {
                return vec![RawCandidate {
                    root: redup.root,
                    operation: "reduplication_exact",
                    base_weight: 0.98,
                }];
            }
        }

        let mut candidates = Vec::new();

        if dict.contains(&clean_word) {
            candidates.push(RawCandidate {
                root: clean_word.clone(),
                operation: "exact_root",
                base_weight: 0.99,
            });
        }

        let infix_cands = strip_infixes(&clean_word, dict);
        for cand in infix_cands {
            candidates.push(RawCandidate {
                root: cand.root,
                operation: "infix",
                base_weight: cand.weight,
            });
        }

        // 1. Circonfixes couplés (f-...-ana, faha-...-ana, fam-...-ana, fan-...-ana, etc.)
        let circum_cands = strip_circumfixes(&clean_word, dict);
        for cand in circum_cands {
            candidates.push(RawCandidate {
                root: cand.root,
                operation: "circumfix_coupled",
                base_weight: cand.weight,
            });
        }

        let prefix_cands = generate_prefix_candidates(&clean_word, dict);
        for cand in &prefix_cands {
            let in_dict = dict.contains(&cand.root);
            let weight = if in_dict {
                cand.weight
            } else {
                let word_has_suffix = clean_word.ends_with("ana") || clean_word.ends_with("ina") || clean_word.ends_with("ena");
                let cand_has_suffix = cand.root.ends_with("ana") || cand.root.ends_with("ina") || cand.root.ends_with("ena");
                if word_has_suffix && cand_has_suffix { cand.weight * 0.75 } else { cand.weight }
            };

            candidates.push(RawCandidate {
                root: cand.root.clone(),
                operation: "prefix_nasal_mutation",
                base_weight: weight,
            });

            let sub_suffixes = strip_suffixes(&cand.root, dict);
            for s_cand in sub_suffixes {
                candidates.push(RawCandidate {
                    root: s_cand.root,
                    operation: "prefix_then_suffix",
                    base_weight: cand.weight * s_cand.weight * 0.95,
                });
            }

            for restored in restore_morphophonemic_endings(&cand.root, dict) {
                candidates.push(RawCandidate {
                    root: restored,
                    operation: "prefix_then_morphophonemic",
                    base_weight: cand.weight * 0.85,
                });
            }
        }

        let suffix_cands = strip_suffixes(&clean_word, dict);
        for cand in suffix_cands {
            candidates.push(RawCandidate {
                root: cand.root.clone(),
                operation: "suffix_restoration",
                base_weight: cand.weight,
            });

            let sub_prefixes = generate_prefix_candidates(&cand.root, dict);
            for p_cand in sub_prefixes {
                candidates.push(RawCandidate {
                    root: p_cand.root.clone(),
                    operation: "suffix_then_prefix",
                    base_weight: p_cand.weight * cand.weight * 0.95,
                });

                for restored in restore_morphophonemic_endings(&p_cand.root, dict) {
                    candidates.push(RawCandidate {
                        root: restored,
                        operation: "circumfix_fully_restored",
                        base_weight: p_cand.weight * cand.weight * 0.90,
                    });
                }
            }
        }

        if candidates.is_empty() {
            candidates.push(RawCandidate {
                root: clean_word,
                operation: "identity_fallback",
                base_weight: 0.1,
            });
        }

        candidates
    }
}

fn restore_morphophonemic_endings(base: &str, dict: &FstDictionary) -> Vec<String> {
    let mut results = Vec::new();
    if base.len() < 2 {
        return results;
    }

    if base.ends_with('t') {
        let cand = format!("{}ra", base);
        if dict.contains(&cand) {
            results.push(cand);
        }
    } else if base.ends_with('r') {
        let cand = format!("{}tra", &base[..base.len() - 1]);
        if dict.contains(&cand) {
            results.push(cand);
        }
    } else if base.ends_with('h') {
        let cand = format!("{}ka", &base[..base.len() - 1]);
        if dict.contains(&cand) {
            results.push(cand);
        }
    } else if base.ends_with("en") && base.len() >= 4 {
        let cand_y = format!("{}y", &base[..base.len() - 2]);
        if dict.contains(&cand_y) {
            results.push(cand_y);
        }
    } else if base.ends_with("ez") && base.len() >= 4 {
        let cand_hy = format!("{}hy", &base[..base.len() - 2]);
        if dict.contains(&cand_hy) {
            results.push(cand_hy);
        }
    } else if base.ends_with('i') && base.len() >= 3 {
        let cand_y = format!("{}y", &base[..base.len() - 1]);
        if dict.contains(&cand_y) {
            results.push(cand_y);
        }
    } else {
        for suffix in &["y", "a", "na", "tra", "ka", "e", "o"] {
            let cand = format!("{}{}", base, suffix);
            if dict.contains(&cand) {
                results.push(cand);
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morphology_pipeline_basic() {
        let dict = crate::dictionary::default_dictionary();

        let cands = MorphologyEngine::analyze("manoratra", dict);
        assert!(cands.iter().any(|c| c.root == "soratra"));

        let cands = MorphologyEngine::analyze("tsaratsara", dict);
        assert!(cands.iter().any(|c| c.root == "tsara"));

        let cands = MorphologyEngine::analyze("harem-pirenena", dict);
        assert!(cands.iter().any(|c| c.root == "harena_firenena"));

        let cands = MorphologyEngine::analyze("fampianarana", dict);
        assert!(cands.iter().any(|c| c.root == "anatra"));
    }
}
