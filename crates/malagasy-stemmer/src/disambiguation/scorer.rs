use crate::dictionary::FstDictionary;
use crate::morphology::RawCandidate;

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateScore {
    pub root: String,
    pub score: f64,
    pub operation: &'static str,
    pub in_dictionary: bool,
}

pub struct ViterbiScorer;

impl ViterbiScorer {
    pub fn select_best(candidates: Vec<RawCandidate>, dict: &FstDictionary) -> CandidateScore {
        Self::select_best_for_word("", candidates, dict)
    }

    pub fn select_best_for_word(word: &str, candidates: Vec<RawCandidate>, dict: &FstDictionary) -> CandidateScore {
        if candidates.is_empty() {
            return CandidateScore {
                root: String::new(),
                score: 0.0,
                operation: "none",
                in_dictionary: false,
            };
        }

        let word_clean = word.trim().to_lowercase();
        let word_len = word_clean.chars().count();
        let has_circumfix_shape = (word_clean.starts_with("fa")
            || word_clean.starts_with("fi")
            || word_clean.starts_with("faha")
            || word_clean.starts_with("famp")
            || word_clean.starts_with("ha"))
            && (word_clean.ends_with("ana")
                || word_clean.ends_with("ina")
                || word_clean.ends_with("ena")
                || word_clean.ends_with("ona"));

        let mut scored_candidates: Vec<CandidateScore> = candidates
            .into_iter()
            .map(|c| {
                let in_dict = dict.contains(&c.root);
                let dict_factor = if in_dict { 1.0 } else { 0.20 };
                let mut op_factor = c.base_weight;

                if has_circumfix_shape {
                    if c.operation == "circumfix_coupled" || c.operation == "circumfix_fully_restored" {
                        op_factor *= 1.15;
                    } else if c.operation == "prefix_nasal_mutation"
                        && (c.root.ends_with("ana") || c.root.ends_with("ina") || c.root.ends_with("ena") || c.root.ends_with("ona"))
                    {
                        // Suffix was not stripped, only prefix was removed
                        op_factor *= 0.65;
                    }
                }

                // If word ends with -sana (protective s), prioritize f- roots over v- roots
                if word_clean.ends_with("sana") && c.root.starts_with('f') {
                    op_factor *= 1.05;
                }

                // If word ends with -ena or -ezana, prioritize -y ending roots (e.g. voly for fambolena, fehy for famehezana)
                if (word_clean.ends_with("ena") || word_clean.ends_with("ezana")) && c.root.ends_with('y') {
                    op_factor *= 1.10;
                }

                let len = c.root.chars().count();
                let mut len_factor = match len {
                    0..=1 => 0.01,
                    2 => {
                        if in_dict && word_len <= 4 {
                            0.95
                        } else if in_dict {
                            0.50
                        } else {
                            0.05
                        }
                    }
                    3 => {
                        if in_dict && word_len >= 8 {
                            0.70
                        } else if in_dict {
                            0.98
                        } else {
                            0.30
                        }
                    }
                    4..=8 => 1.0,
                    9..=14 => 0.90,
                    _ => 0.70,
                };

                // Penalize severe over-stemming when original word is long
                if word_len >= 7 && len <= 3 && in_dict {
                    len_factor *= 0.70;
                }

                let phonotactic_factor = if has_valid_malagasy_ending(&c.root) {
                    1.0
                } else if in_dict {
                    0.95
                } else {
                    0.15
                };

                let total_score = dict_factor * 0.40 + op_factor * 0.35 + len_factor * 0.15 + phonotactic_factor * 0.10;

                CandidateScore {
                    root: c.root,
                    score: total_score,
                    operation: c.operation,
                    in_dictionary: in_dict,
                }
            })
            .collect();

        // Sort candidates:

        scored_candidates.sort_by(|a, b| {
            b.in_dictionary
                .cmp(&a.in_dictionary)
                .then_with(|| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal))
                .then_with(|| {
                    if word_len > 0 {
                        let target_len = ((word_len as f64) * 0.65).round() as isize;
                        let diff_a = ((a.root.chars().count() as isize) - target_len).abs();
                        let diff_b = ((b.root.chars().count() as isize) - target_len).abs();
                        diff_a.cmp(&diff_b)
                    } else {
                        b.root.len().cmp(&a.root.len())
                    }
                })
        });

        scored_candidates.remove(0)
    }
}

#[inline]
fn has_valid_malagasy_ending(word: &str) -> bool {
    if word.is_empty() {
        return false;
    }
    word.ends_with('a')
        || word.ends_with('e')
        || word.ends_with('i')
        || word.ends_with('o')
        || word.ends_with('y')
        || word.ends_with("tra")
        || word.ends_with("ka")
        || word.ends_with("na")
        || word.ends_with("ra")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scorer_prefers_dictionary_root() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = vec![
            RawCandidate {
                root: "toratra".to_string(), // Inexistant
                operation: "prefix_nasal_mutation",
                base_weight: 0.9,
            },
            RawCandidate {
                root: "soratra".to_string(), // Existe dans le dictionnaire
                operation: "prefix_nasal_mutation",
                base_weight: 0.9,
            },
        ];

        let best = ViterbiScorer::select_best_for_word("manoratra", candidates, dict);
        assert_eq!(best.root, "soratra");
        assert!(best.in_dictionary);
        assert!(best.score > 0.8);
    }
}
