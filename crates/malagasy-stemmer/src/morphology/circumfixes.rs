//! Détection et désaffixation conjointe des circonfixes nominaux et circonstanciels.

use crate::dictionary::FstDictionary;
use crate::morphology::reduplication::strip_reduplication;
use crate::morphology::rules::{NasalMutationRule, NASAL_MUTATIONS};

#[derive(Debug, Clone)]
pub struct CircumfixCandidate {
    pub root: String,
    pub prefix: &'static str,
    pub suffix: &'static str,
    pub weight: f64,
}

pub static CIRCUMFIX_PREFIXES_SIMPLE: &[&str] = &[
    "mpampif", "fampif", "mampif", "nampif", "hampif",
    "mpamaha", "famaha", "mamaha", "namaha", "hamaha",
    "mpampi", "fampi", "mampi", "nampi", "hampi",
    "mpamp", "famp", "mamp", "namp", "hamp",
    "mpanka", "fanka", "manka", "nanka", "hanka",
    "mpana", "fana", "mana", "nana", "hana",
    "faha", "mpaha", "maha", "tafa", "fif", "mpi", "fi", "fa", "ha",
];

pub static CIRCUMFIX_SUFFIXES: &[&str] = &[
    "antsika", "anareo", "anare", "tsika", "nareo", "anay", "areo",
    "ehena", "ehana", "ahana", "ahina",
    "erina", "erana", "ohina", "orina", "orana",
    "ezana", "izana", "ozana",
    "zina", "sina", "vina", "hina", "tina", "rina", "fina",
    "zana", "sana",
    "ana", "ena", "ina", "ona",
];

/// Analyse un mot pour extraire des racines candidates par désaffixation conjointe de circonfixes.
pub fn strip_circumfixes(word: &str, dict: &FstDictionary) -> Vec<CircumfixCandidate> {
    let mut candidates = Vec::new();

    // Circonfixes require a paired suffix (e.g. -ana, -ena, -ina, -ona, -sana, -zana, etc.)
    let has_circumfix_suffix = CIRCUMFIX_SUFFIXES.iter().any(|&s| word.ends_with(s));
    if !has_circumfix_suffix {
        return candidates;
    }

    // 1. Circonfixes avec mutations nasales (fam-...-ana, fan-...-ana, fang-...-ana, etc.)
    for nasal_rule in NASAL_MUTATIONS {
        if let Some(after_pfx) = word.strip_prefix(nasal_rule.prefix) {
            if after_pfx.len() < 3 {
                continue;
            }

            for &suffix in CIRCUMFIX_SUFFIXES {
                if let Some(core) = after_pfx.strip_suffix(suffix) {
                    if core.is_empty() {
                        continue;
                    }

                    // Tentative avec réduplication interne : ex: famaingavaingana -> core = "aingavaing"
                    // On teste si une restauration nasale sur 'core' ou son dé-dédoublement fonctionne
                    let candidate_bases = generate_nasal_restorations(core, nasal_rule);

                    for base in &candidate_bases {
                        // Restauration de terminaison directe ou morphophonémique
                        for restored in restore_circumfix_endings(base, suffix, dict) {
                            candidates.push(CircumfixCandidate {
                                root: restored,
                                prefix: nasal_rule.prefix,
                                suffix,
                                weight: 1.0,
                            });
                        }

                        // Test si la base elle-même est une réduplication : vaingavainga -> vainga
                        if let Some(redup) = strip_reduplication(base) {
                            for restored in restore_circumfix_endings(&redup.root, suffix, dict) {
                                candidates.push(CircumfixCandidate {
                                    root: restored,
                                    prefix: nasal_rule.prefix,
                                    suffix,
                                    weight: 0.98,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. Circonfixes simples (faha-...-ana, famp-...-ana, fi-...-ana, etc.)
    for &prefix in CIRCUMFIX_PREFIXES_SIMPLE {
        if let Some(after_pfx) = word.strip_prefix(prefix) {
            if after_pfx.len() < 2 {
                continue;
            }

            let is_nasal_prefixed = (prefix == "fa" && (word.starts_with("fam") || word.starts_with("fan") || word.starts_with("fang")))
                || (prefix == "ma" && (word.starts_with("mam") || word.starts_with("man") || word.starts_with("mang")))
                || (prefix == "ha" && (word.starts_with("ham") || word.starts_with("han") || word.starts_with("hang")))
                || (prefix == "mp" && (word.starts_with("mpam") || word.starts_with("mpan") || word.starts_with("mpang")))
                || (prefix == "fi" && (word.starts_with("famp") || word.starts_with("fampi") || word.starts_with("fampif")));

            if is_nasal_prefixed && !dict.contains(after_pfx) {
                continue;
            }

            for &suffix in CIRCUMFIX_SUFFIXES {
                if let Some(core) = after_pfx.strip_suffix(suffix) {
                    if core.len() < 2 {
                        continue;
                    }

                    for restored in restore_circumfix_endings(core, suffix, dict) {
                        candidates.push(CircumfixCandidate {
                            root: restored,
                            prefix,
                            suffix,
                            weight: 0.99,
                        });
                    }

                    if let Some(redup) = strip_reduplication(core) {
                        for restored in restore_circumfix_endings(&redup.root, suffix, dict) {
                            candidates.push(CircumfixCandidate {
                                root: restored,
                                prefix,
                                suffix,
                                weight: 0.98,
                            });
                        }
                    }
                }
            }
        }
    }

    candidates
}

fn generate_nasal_restorations(core: &str, rule: &NasalMutationRule) -> Vec<String> {
    let mut bases = Vec::new();

    // 1. Voyelles directes : core commence par voyelle -> on préfixe par les consonnes sous-jacentes
    let vowel_restorations: &[&str] = if core.ends_with('s') && rule.prefix.ends_with('m') {
        &["f", "v", "p", "b"]
    } else {
        rule.vowel_restorations
    };

    for &cons in vowel_restorations {
        bases.push(format!("{}{}", cons, core));
    }

    // 2. Mutations consonantiques : dr -> r, b -> v, d -> l, j -> nj, etc.
    for &(surface, underlying) in rule.consonant_mutations {
        if let Some(rest) = core.strip_prefix(surface) {
            bases.push(format!("{}{}", underlying, rest));
        }
    }

    // 3. Cas où le core conserve directement sa consonne ou voyelle pour préfixes en -an
    if rule.prefix.ends_with('n') || !core.starts_with(|c: char| matches!(c, 'a' | 'e' | 'i' | 'o' | 'y')) {
        bases.push(core.to_string());
    }

    bases
}

/// Restaure les alternances finales morphophonémiques dues à l'adjonction de suffixes nominaux.
fn restore_circumfix_endings(base: &str, suffix: &str, dict: &FstDictionary) -> Vec<String> {
    let mut results = Vec::new();
    if base.is_empty() {
        return results;
    }

    // A. Morphophonemic consonant alternations (highest linguistic priority for truncated stems)
    if base.ends_with("ra") && base.len() >= 3 {
        let cand = format!("{}tra", &base[..base.len() - 2]);
        if dict.contains(&cand) && !results.contains(&cand) {
            results.push(cand);
        }
    } else if base.ends_with('r') && base.len() >= 2 {
        let cand = format!("{}tra", &base[..base.len() - 1]);
        if dict.contains(&cand) && !results.contains(&cand) {
            results.push(cand);
        }
    } else if base.ends_with('t') {
        let cand = format!("{}ra", base);
        if dict.contains(&cand) && !results.contains(&cand) {
            results.push(cand);
        }
    } else if base.ends_with("eh") && base.len() >= 4 {
        let cand = format!("{}ika", &base[..base.len() - 2]);
        if dict.contains(&cand) && !results.contains(&cand) {
            results.push(cand);
        }
    } else if base.ends_with("ha") && base.len() >= 3 {
        let cand = format!("{}ka", &base[..base.len() - 2]);
        if dict.contains(&cand) && !results.contains(&cand) {
            results.push(cand);
        }
    } else if base.ends_with('h') && base.len() >= 2 {
        let cand = format!("{}ka", &base[..base.len() - 1]);
        if dict.contains(&cand) && !results.contains(&cand) {
            results.push(cand);
        }
    } else if base.ends_with("ez") && base.len() >= 4 {
        let cand = format!("{}hy", &base[..base.len() - 2]);
        if dict.contains(&cand) && !results.contains(&cand) {
            results.push(cand);
        }
        let cand_y = format!("{}y", &base[..base.len() - 2]);
        if dict.contains(&cand_y) && !results.contains(&cand_y) {
            results.push(cand_y);
        }
    } else if base.ends_with("en") && base.len() >= 4 {
        let cand_ina = format!("{}ina", &base[..base.len() - 2]);
        if dict.contains(&cand_ina) && !results.contains(&cand_ina) {
            results.push(cand_ina);
        }
        let cand_y = format!("{}y", &base[..base.len() - 2]);
        if dict.contains(&cand_y) && !results.contains(&cand_y) {
            results.push(cand_y);
        }
    } else if base.ends_with("an") && base.len() >= 4 {
        let cand_a = format!("{}a", base);
        if dict.contains(&cand_a) && !results.contains(&cand_a) {
            results.push(cand_a);
        }
    } else if base.ends_with('i') && base.len() >= 3 {
        let cand_y = format!("{}y", &base[..base.len() - 1]);
        if dict.contains(&cand_y) && !results.contains(&cand_y) {
            results.push(cand_y);
        }
    } else if (base.ends_with('s') || base.ends_with('z') || base.ends_with('v')) && base.len() >= 3 {
        let cand = &base[..base.len() - 1];
        if dict.contains(cand) && !results.contains(&cand.to_string()) {
            results.push(cand.to_string());
        }
    }

    // B. Direct match of the base in dictionary (e.g. voa, vango, fono, ala, adina, vilana, tamana)
    if dict.contains(base) && !results.contains(&base.to_string()) {
        results.push(base.to_string());
    }

    // C. If no root found yet, try regular ending restorations
    if results.is_empty() {
        let endings: &[&str] = if suffix == "ena" || suffix == "ezana" {
            &["y", "tra", "ka", "na", "a", "o", "e", "ra", "ina", "ana", "ona"]
        } else {
            &["tra", "ka", "na", "y", "a", "o", "e", "ra", "ina", "ana", "ona", "ena"]
        };

        for &ending in endings {
            let cand = format!("{}{}", base, ending);
            if dict.contains(&cand) && !results.contains(&cand) {
                results.push(cand);
            }
        }
    }

    // 4. Terminaisons de voyelles ouvertes (a, o, y, e) avec suffixe en -zana / -sana (fahavoazana -> voa)
    if suffix.starts_with('z') || suffix.starts_with('s') || suffix.starts_with('h') || suffix.starts_with('t') || suffix.starts_with('r') {
        if dict.contains(base) && !results.contains(&base.to_string()) {
            results.push(base.to_string());
        }
        for ending in &["a", "o", "y", "e", "tra", "ka", "na"] {
            let cand = format!("{}{}", base, ending);
            if dict.contains(&cand) && !results.contains(&cand) {
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
    fn test_fahavoazana() {
        let dict = crate::dictionary::default_dictionary();
        let cands = strip_circumfixes("fahavoazana", dict);
        assert!(cands.iter().any(|c| c.root == "voa"), "fahavoazana -> voa");
    }

    #[test]
    fn test_famadidirana() {
        let dict = crate::dictionary::default_dictionary();
        let cands = strip_circumfixes("famadidirana", dict);
        assert!(cands.iter().any(|c| c.root == "vadiditra"), "famadidirana -> vadiditra");
    }

    #[test]
    fn test_famaritana() {
        let dict = crate::dictionary::default_dictionary();
        let cands = strip_circumfixes("famaritana", dict);
        assert!(cands.iter().any(|c| c.root == "faritra"), "famaritana -> faritra");
    }

    #[test]
    fn test_famehezana() {
        let dict = crate::dictionary::default_dictionary();
        let cands = strip_circumfixes("famehezana", dict);
        assert!(cands.iter().any(|c| c.root == "fehy"), "famehezana -> fehy");
    }
}
