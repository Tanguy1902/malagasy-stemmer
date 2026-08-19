#[derive(Debug, Clone)]
pub struct ReduplicationResult {
    pub root: String,
    pub kind: ReduplicationKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReduplicationKind {
    Exact,
    WithLiaison,
}

pub fn strip_reduplication(word: &str) -> Option<ReduplicationResult> {
    let len = word.len();

    if len < 4 {
        return None;
    }

    if len % 2 == 0 {
        let mid = len / 2;
        let first_half = &word[..mid];
        let second_half = &word[mid..];
        if first_half == second_half && first_half.len() >= 2 {
            return Some(ReduplicationResult {
                root: first_half.to_string(),
                kind: ReduplicationKind::Exact,
            });
        }
    }

    let char_count = word.chars().count();
    if char_count >= 6 {
        let mid_char = char_count / 2;

        for offset in [0i32, -1, 1] {
            let cut_pos = (mid_char as i32 + offset) as usize;
            if cut_pos < 3 || cut_pos > char_count - 3 {
                continue;
            }

            let byte_pos: usize = word.chars().take(cut_pos).map(|c| c.len_utf8()).sum();
            let first = &word[..byte_pos];
            let second = &word[byte_pos..];

            let common_prefix_len = first
                .chars()
                .zip(second.chars())
                .take_while(|(a, b)| a == b)
                .count();

            if common_prefix_len >= 3 && common_prefix_len >= first.chars().count().min(second.chars().count()) - 1 {
                return Some(ReduplicationResult {
                    root: first.to_string(),
                    kind: ReduplicationKind::WithLiaison,
                });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_reduplication() {
        let result = strip_reduplication("tsaratsara").unwrap();
        assert_eq!(result.root, "tsara");
        assert_eq!(result.kind, ReduplicationKind::Exact);
    }

    #[test]
    fn test_moramora() {
        let result = strip_reduplication("moramora").unwrap();
        assert_eq!(result.root, "mora");
        assert_eq!(result.kind, ReduplicationKind::Exact);
    }

    #[test]
    fn test_mamaky_is_not_reduplication() {
        assert!(strip_reduplication("mamaky").is_none());
    }

    #[test]
    fn test_no_reduplication() {
        assert!(strip_reduplication("soratra").is_none());
        assert!(strip_reduplication("ab").is_none());
    }
}
