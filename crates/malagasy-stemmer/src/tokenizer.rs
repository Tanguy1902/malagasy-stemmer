use crate::stemmer::{MalagasyStemmer, StemResult};
use crate::stopwords::is_stopword;

pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let normalized = normalize_typography(text);

    for raw_word in extract_words_with_symbols(&normalized) {
        if raw_word.is_empty() {
            continue;
        }

        if raw_word.contains('\'') {
            let split_tokens = split_apostrophe_contraction(&raw_word);
            tokens.extend(split_tokens);
        } else {
            tokens.push(raw_word.to_lowercase());
        }
    }

    tokens
}

fn normalize_typography(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '’' | '‘' | '`' | '´' => '\'',
            '–' | '—' | '\u{2011}' => '-',
            other => other,
        })
        .collect()
}

fn extract_words_with_symbols(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    let mut i = 0;
    while i < len {
        let c = chars[i];

        if c.is_alphabetic() {
            current.push(c);
        } else if (c == '-' || c == '\'') && !current.is_empty() && i + 1 < len && chars[i + 1].is_alphabetic() {
            current.push(c);
        } else {
            if !current.is_empty() {
                let cleaned = current.trim_matches(|p| p == '-' || p == '\'').to_string();
                if !cleaned.is_empty() {
                    words.push(cleaned);
                }
                current.clear();
            }
        }
        i += 1;
    }

    if !current.is_empty() {
        let cleaned = current.trim_matches(|p| p == '-' || p == '\'').to_string();
        if !cleaned.is_empty() {
            words.push(cleaned);
        }
    }

    words
}

fn split_apostrophe_contraction(word: &str) -> Vec<String> {
    let lower = word.to_lowercase();
    let parts: Vec<&str> = lower.split('\'').filter(|p| !p.is_empty()).collect();

    if parts.is_empty() {
        return Vec::new();
    }

    if parts.len() == 1 {
        return vec![parts[0].to_string()];
    }

    let mut result = Vec::new();

    for (idx, &part) in parts.iter().enumerate() {
        if idx == 0 {
            let is_followed_by_ny = parts.get(idx + 1) == Some(&"ny");
            if is_followed_by_ny && part.len() >= 3 && part.ends_with('n') {
                if part == "amin" || part == "tamin" || part == "an" {
                    result.push(part.to_string());
                } else if part.ends_with("on") || part.ends_with("in") || part.ends_with("en") {
                    let stem_guess = if part.ends_with("in") {
                        format!("{}y", &part[..part.len() - 2])
                    } else {
                        part[..part.len() - 1].to_string()
                    };
                    result.push(stem_guess);
                } else if part.ends_with("an") {
                    result.push(format!("{}a", part));
                } else {
                    result.push(part.to_string());
                }
            } else {
                result.push(part.to_string());
            }
        } else {
            result.push(part.to_string());
        }
    }

    result
}

/// Découpe un texte malgache et extrait les racines (*fototeny*) de chaque mot.
///
/// ```
/// use malagasy_stemmer::tokenizer::tokenize_and_stem;
/// let text = "Nanoratra taratasy ho an'ny mpianatra izy";
/// let roots = tokenize_and_stem(text, true);
/// assert!(roots.contains(&"soratra".to_string()));
/// assert!(roots.contains(&"anatra".to_string()));
/// ```
pub fn tokenize_and_stem(text: &str, remove_stopwords: bool) -> Vec<String> {
    let stemmer = MalagasyStemmer::new();
    let tokens = tokenize(text);

    tokens
        .into_iter()
        .filter(|token| !remove_stopwords || !is_stopword(token))
        .map(|token| stemmer.stem(&token))
        .filter(|root| !root.is_empty())
        .collect()
}

pub fn tokenize_and_stem_with_details(
    text: &str,
    remove_stopwords: bool,
) -> Vec<StemResult> {
    let stemmer = MalagasyStemmer::new();
    let tokens = tokenize(text);

    tokens
        .into_iter()
        .filter(|token| !remove_stopwords || !is_stopword(token))
        .map(|token| stemmer.stem_with_details(&token))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let text = "Mianatra teny malagasy isika.";
        let tokens = tokenize(text);
        assert_eq!(tokens, vec!["mianatra", "teny", "malagasy", "isika"]);
    }

    #[test]
    fn test_tokenize_preserves_compounds() {
        let text = "Ny harem-pirenena sy ny tanan-dehibe.";
        let tokens = tokenize(text);
        assert_eq!(tokens, vec!["ny", "harem-pirenena", "sy", "ny", "tanan-dehibe"]);
    }

    #[test]
    fn test_tokenize_apostrophe_contractions() {
        let text = "Niresaka tamin'ny mpianatra sy momba an'i Madagasikara izy.";
        let tokens = tokenize(text);
        assert_eq!(
            tokens,
            vec!["niresaka", "tamin", "ny", "mpianatra", "sy", "momba", "an", "i", "madagasikara", "izy"]
        );
    }

    #[test]
    fn test_tokenize_and_stem_with_stopwords_removal() {
        let text = "Nanoratra boky sy tantara momba ny fampianarana izy.";
        let roots = tokenize_and_stem(text, true);

        // "sy", "momba", "ny", "izy" sont des stopwords et doivent être éliminés
        assert!(!roots.contains(&"sy".to_string()));
        assert!(!roots.contains(&"ny".to_string()));

        // Les racines attendues
        assert!(roots.contains(&"soratra".to_string()));
        assert!(roots.contains(&"boky".to_string()));
        assert!(roots.contains(&"tantara".to_string()));
        assert!(roots.contains(&"anatra".to_string()));
    }

    #[test]
    fn test_tokenize_and_stem_compounds() {
        let text = "Miaro ny harem-pirenena sy ny tanan-dehibe isika.";
        let roots = tokenize_and_stem(text, true);
        assert!(roots.contains(&"harena_firenena".to_string()));
        assert!(roots.contains(&"tanana_lehibe".to_string()));
    }
}

