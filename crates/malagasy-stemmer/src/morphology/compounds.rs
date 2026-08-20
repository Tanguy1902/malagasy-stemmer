use crate::dictionary::FstDictionary;
use crate::morphology::rules::SANDHI_MUTATIONS;

#[derive(Debug, Clone)]
pub struct CompoundResult {
    pub parts: Vec<String>,
    pub joined: String,
}

pub fn stem_compound(word: &str, dict: &FstDictionary) -> Option<CompoundResult> {
    if !word.contains('-') {
        return None;
    }

    let parts: Vec<&str> = word.splitn(2, '-').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return None;
    }

    let left = parts[0];
    let right = parts[1];

    if left == "maha" || left == "faha" || left == "tafa" {
        let restored_right = restore_sandhi(right, dict);
        return Some(CompoundResult {
            parts: vec![restored_right.clone()],
            joined: restored_right,
        });
    }

    let restored_left = restore_compound_left(left, dict);
    let mut restored_right = restore_sandhi(right, dict);

    if !dict.contains(&restored_right) || restored_right.starts_with("fam") || restored_right.starts_with("fan") {
        let circum_cands = crate::morphology::circumfixes::strip_circumfixes(&restored_right, dict);
        if let Some(best) = circum_cands.iter().find(|c| dict.contains(&c.root)) {
            restored_right = best.root.clone();
        }
    }

    let joined = format!("{}_{}", restored_left, restored_right);

    Some(CompoundResult {
        parts: vec![restored_left, restored_right],
        joined,
    })
}

fn restore_compound_left(word: &str, dict: &FstDictionary) -> String {
    if word.is_empty() {
        return word.to_string();
    }

    if word == "ara" {
        return "araka".to_string();
    } else if word == "isan" {
        return "isa".to_string();
    } else if word == "mpiara" {
        return "miara".to_string();
    } else if word == "an" {
        return "any".to_string();
    } else if word == "rao" {
        return "raoka".to_string();
    } else if word == "tanan" {
        return "tanana".to_string();
    }

    if word.ends_with('m') || word.ends_with('n') {
        let base = &word[..word.len() - 1];

        if dict.contains(base) {
            return base.to_string();
        }

        let cand_na = format!("{}na", base);
        if dict.contains(&cand_na) {
            return cand_na;
        }

        let cand_a = format!("{}a", word);
        if dict.contains(&cand_a) {
            return cand_a;
        }
    }

    if dict.contains(word) {
        return word.to_string();
    }

    for suffix in &["ka", "na", "a", "ana", "tra"] {
        let cand = format!("{}{}", word, suffix);
        if dict.contains(&cand) {
            return cand;
        }
    }

    word.to_string()
}

fn restore_sandhi(word: &str, dict: &FstDictionary) -> String {
    if word.is_empty() {
        return word.to_string();
    }

    // Si le mot est déjà une racine courte canonique valide (ex: dia, tany, vato)
    if dict.contains(word) && !word.starts_with("dehi") && !word.starts_with("drano") && !word.starts_with("piren") && !word.starts_with("dolana") {
        return word.to_string();
    }

    for rule in SANDHI_MUTATIONS {
        if word.starts_with(rule.surface) {
            let candidate = format!("{}{}", rule.lexical, &word[rule.surface.len()..]);
            if dict.contains(&candidate) {
                return candidate;
            }
        }
    }

    if dict.contains(word) {
        return word.to_string();
    }

    for rule in SANDHI_MUTATIONS {
        if word.starts_with(rule.surface) {
            return format!("{}{}", rule.lexical, &word[rule.surface.len()..]);
        }
    }

    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harem_pirenena() {
        let dict = crate::dictionary::default_dictionary();
        let result = stem_compound("harem-pirenena", dict).unwrap();
        assert_eq!(result.parts[0], "harena");
        assert_eq!(result.parts[1], "firenena");
        assert_eq!(result.joined, "harena_firenena");
    }

    #[test]
    fn test_tanan_dehibe() {
        let dict = crate::dictionary::default_dictionary();
        let result = stem_compound("tanan-dehibe", dict).unwrap();
        assert_eq!(result.parts[0], "tanana");
        assert_eq!(result.parts[1], "lehibe");
        assert_eq!(result.joined, "tanana_lehibe");
    }

    #[test]
    fn test_ara_compound() {
        let dict = crate::dictionary::default_dictionary();
        let result = stem_compound("ara-potoana", dict).unwrap();
        assert_eq!(result.parts[0], "araka");
        assert_eq!(result.parts[1], "fotoana");
        assert_eq!(result.joined, "araka_fotoana");
    }

    #[test]
    fn test_no_compound() {
        let dict = crate::dictionary::default_dictionary();
        assert!(stem_compound("soratra", dict).is_none());
    }
}
