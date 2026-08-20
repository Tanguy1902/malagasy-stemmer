use crate::dictionary::FstDictionary;

#[derive(Debug, Clone)]
pub struct InfixCandidate {
    pub root: String,
    pub infix: String,
    pub weight: f64,
}

static INFIXES: &[(&str, f64)] = &[
    ("in", 0.8),
    ("om", 0.7),
];

pub fn strip_infixes(word: &str, dict: &FstDictionary) -> Vec<InfixCandidate> {
    let mut candidates = Vec::new();

    if word.len() < 4 {
        return candidates;
    }

    let chars: Vec<char> = word.chars().collect();

    for &(infix, weight) in INFIXES {
        let infix_len = infix.len();

        if word.len() > 1 + infix_len {
            let after_first = &word[chars[0].len_utf8()..];
            if let Some(rest) = after_first.strip_prefix(infix) {
                let candidate: String = std::iter::once(chars[0]).collect::<String>() + rest;
                if candidate.len() >= 3 {
                    let in_dict = dict.contains(&candidate);
                    candidates.push(InfixCandidate {
                        root: candidate,
                        infix: infix.to_string(),
                        weight: if in_dict { weight } else { weight * 0.4 },
                    });
                }

                if !rest.starts_with(|c: char| matches!(c, 'a' | 'e' | 'i' | 'o' | 'y')) {
                    for &v in &["a", "e", "i", "o"] {
                        let cand_v = format!("{}{}{}", chars[0], v, rest);
                        if dict.contains(&cand_v) {
                            candidates.push(InfixCandidate {
                                root: cand_v,
                                infix: infix.to_string(),
                                weight,
                            });
                        }
                    }
                }
            }
        }

        if chars.len() > 2 + infix_len {
            let prefix_len: usize = chars[..2].iter().map(|c| c.len_utf8()).sum();
            let after_two = &word[prefix_len..];
            if let Some(rest) = after_two.strip_prefix(infix) {
                let candidate: String = chars[..2].iter().collect::<String>() + rest;
                if candidate.len() >= 3 && dict.contains(&candidate) {
                    candidates.push(InfixCandidate {
                        root: candidate,
                        infix: infix.to_string(),
                        weight: weight * 0.8,
                    });
                }
            }
        }
    }

    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_in_infix() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = strip_infixes("vinaky", dict);
        assert!(
            candidates.iter().any(|c| c.root == "vaky"),
            "vinaky doit produire 'vaky' (retrait de -in-)"
        );
    }

    #[test]
    fn test_short_word_no_infix() {
        let dict = crate::dictionary::default_dictionary();
        let candidates = strip_infixes("fo", dict);
        assert!(candidates.is_empty());
    }
}
