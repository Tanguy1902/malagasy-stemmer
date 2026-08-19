
pub mod dictionary;
pub mod disambiguation;
pub mod fuzzy;
pub mod morphology;
pub mod stemmer;
pub mod stopwords;
pub mod tokenizer;

#[cfg(feature = "tantivy")]
pub mod tantivy_integration;

pub use dictionary::{default_dictionary, FstDictionary, FuzzyMatch};
pub use disambiguation::{CandidateScore, ViterbiScorer};
pub use fuzzy::fuzzy_root_lookup;
pub use stemmer::{stem, MalagasyStemmer, StemResult};
pub use stopwords::is_stopword;
pub use tokenizer::{tokenize, tokenize_and_stem, tokenize_and_stem_with_details};

#[cfg(feature = "tantivy")]
pub use tantivy_integration::{create_malagasy_analyzer, MalagasyStemFilter, MalagasyStemTokenStream};

