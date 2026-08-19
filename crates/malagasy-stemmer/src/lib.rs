//! # Malagasy Stemmer (`malagasy-stemmer`)
//!
//! Stemmer morphologique et tokenizer pour la langue malgache (*Teny Malagasy*).
//!
//! ```rust
//! use malagasy_stemmer::{stem, tokenize_and_stem, is_stopword};
//!
//! assert_eq!(stem("manoratra"), "soratra");
//! assert_eq!(stem("fampianarana"), "anatra");
//! assert_eq!(stem("harem-pirenena"), "harena_firenena");
//!
//! let text = "Nanoratra taratasy momba ny fampianarana izy";
//! let roots = tokenize_and_stem(text, true);
//! assert_eq!(roots, vec!["soratra", "taratasy", "anatra"]);
//!
//! assert!(is_stopword("dia"));
//! assert!(!is_stopword("vola"));
//! ```

pub mod dictionary;
pub mod disambiguation;
pub mod fuzzy;
pub mod morphology;
pub mod stemmer;
pub mod stopwords;
pub mod tokenizer;

pub use dictionary::{default_dictionary, FstDictionary, FuzzyMatch};
pub use disambiguation::{CandidateScore, ViterbiScorer};
pub use fuzzy::fuzzy_root_lookup;
pub use stemmer::{stem, MalagasyStemmer, StemResult};
pub use stopwords::is_stopword;
pub use tokenizer::{tokenize, tokenize_and_stem, tokenize_and_stem_with_details};
