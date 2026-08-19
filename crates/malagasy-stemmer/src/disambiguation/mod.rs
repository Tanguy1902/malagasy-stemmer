//! Module de désambiguïsation probabiliste inspiré du décodage de Viterbi.

pub mod scorer;

pub use scorer::{CandidateScore, ViterbiScorer};
