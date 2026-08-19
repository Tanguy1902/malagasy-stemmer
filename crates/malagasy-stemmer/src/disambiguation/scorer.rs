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
        if candidates.is_empty() {
            return CandidateScore {
                root: String::new(),
                score: 0.0,
                operation: "none",
                in_dictionary: false,
            };
        }

        let mut scored_candidates: Vec<CandidateScore> = candidates
            .into_iter()
            .map(|c| {
                let in_dict = dict.contains(&c.root);

                let dict_factor = if in_dict { 1.0 } else { 0.20 };
                let op_factor = c.base_weight;

                let len = c.root.chars().count();
                let len_factor = match len {
                    0..=1 => 0.01,
                    2 => if in_dict { 0.9 } else { 0.05 },
                    3 => if in_dict { 0.95 } else { 0.60 },
                    4..=8 => 1.0,
                    9..=12 => 0.85,
                    _ => 0.6,
                };

                let phonotactic_factor = if has_valid_malagasy_ending(&c.root) {
                    1.0
                } else if in_dict {
                    0.95
                } else {
                    0.15
                };

                let total_score = (dict_factor * 0.45 + op_factor * 0.25 + len_factor * 0.15 + phonotactic_factor * 0.15).min(1.0);

                CandidateScore {
                    root: c.root,
                    score: total_score,
                    operation: c.operation,
                    in_dictionary: in_dict,
                }
            })
            .collect();

        scored_candidates.sort_by(|a, b| {
            b.in_dictionary
                .cmp(&a.in_dictionary)
                .then_with(|| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal))
                .then_with(|| b.root.len().cmp(&a.root.len()))
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

        let best = ViterbiScorer::select_best(candidates, dict);
        assert_eq!(best.root, "soratra");
        assert!(best.in_dictionary);
        assert!(best.score > 0.8);
    }
}
