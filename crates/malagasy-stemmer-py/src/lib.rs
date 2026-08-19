//! Bindings PyO3 pour le package Python `malagasy-stemmer`.

use pyo3::prelude::*;
use malagasy_stemmer as core;

/// Résultat détaillé d'un stemming morphologique malgache.
#[pyclass(name = "StemResult")]
#[derive(Clone)]
pub struct PyStemResult {
    #[pyo3(get)]
    pub original: String,
    #[pyo3(get)]
    pub root: String,
    #[pyo3(get)]
    pub confidence: f64,
    #[pyo3(get)]
    pub operation: String,
    #[pyo3(get)]
    pub in_dictionary: bool,
}

#[pymethods]
impl PyStemResult {
    fn __repr__(&self) -> String {
        format!(
            "StemResult(original='{}', root='{}', confidence={:.2}, in_dictionary={})",
            self.original, self.root, self.confidence, self.in_dictionary
        )
    }

    fn __str__(&self) -> String {
        self.root.clone()
    }
}

impl From<core::StemResult> for PyStemResult {
    fn from(res: core::StemResult) -> Self {
        PyStemResult {
            original: res.original,
            root: res.root,
            confidence: res.confidence,
            operation: res.operation,
            in_dictionary: res.in_dictionary,
        }
    }
}

/// Résultat d'une recherche floue (distance de Levenshtein).
#[pyclass(name = "FuzzyMatch")]
#[derive(Clone)]
pub struct PyFuzzyMatch {
    #[pyo3(get)]
    pub word: String,
    #[pyo3(get)]
    pub distance: u32,
}

#[pymethods]
impl PyFuzzyMatch {
    fn __repr__(&self) -> String {
        format!("FuzzyMatch(word='{}', distance={})", self.word, self.distance)
    }
}

impl From<core::FuzzyMatch> for PyFuzzyMatch {
    fn from(m: core::FuzzyMatch) -> Self {
        PyFuzzyMatch {
            word: m.word,
            distance: m.distance,
        }
    }
}

/// Stemmer morphologique haute performance pour la langue malgache.
#[pyclass(name = "MalagasyStemmer")]
pub struct PyMalagasyStemmer {
    inner: core::MalagasyStemmer,
}

#[pymethods]
impl PyMalagasyStemmer {
    #[new]
    fn new() -> Self {
        PyMalagasyStemmer {
            inner: core::MalagasyStemmer::new(),
        }
    }

    /// Extrait la racine (*fototeny*) d'un mot malgache.
    fn stem(&self, word: &str) -> String {
        self.inner.stem(word)
    }

    /// Extrait la racine avec les métadonnées morphologiques détaillées.
    fn stem_with_details(&self, word: &str) -> PyStemResult {
        self.inner.stem_with_details(word).into()
    }

    /// Traite une liste de mots en batch.
    fn stem_batch(&self, words: Vec<String>) -> Vec<String> {
        let refs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        self.inner.stem_batch(&refs)
    }

    /// Traite une liste de mots en batch avec métadonnées complètes.
    fn stem_batch_with_details(&self, words: Vec<String>) -> Vec<PyStemResult> {
        let refs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        self.inner
            .stem_batch_with_details(&refs)
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

/// Extrait la racine (*fototeny*) d'un mot malgache.
#[pyfunction]
fn stem(word: &str) -> String {
    core::stem(word)
}

/// Extrait la racine avec tous les détails d'analyse morphologique et score de confiance.
#[pyfunction]
fn stem_with_details(word: &str) -> PyStemResult {
    let stemmer = core::MalagasyStemmer::new();
    stemmer.stem_with_details(word).into()
}

/// Découpe un texte malgache en tokens (mots) élémentaires.
#[pyfunction]
fn tokenize(text: &str) -> Vec<String> {
    core::tokenize(text)
}

/// Découpe un texte et extrait les racines (*fototeny*) de chaque mot.
#[pyfunction]
#[pyo3(signature = (text, remove_stopwords = true))]
fn tokenize_and_stem(text: &str, remove_stopwords: bool) -> Vec<String> {
    core::tokenize_and_stem(text, remove_stopwords)
}

/// Découpe un texte et retourne les résultats détaillés de chaque token.
#[pyfunction]
#[pyo3(signature = (text, remove_stopwords = true))]
fn tokenize_and_stem_with_details(text: &str, remove_stopwords: bool) -> Vec<PyStemResult> {
    core::tokenize_and_stem_with_details(text, remove_stopwords)
        .into_iter()
        .map(Into::into)
        .collect()
}

/// Vérifie si un mot est un mot vide (stopword) en malgache.
#[pyfunction]
fn is_stopword(word: &str) -> bool {
    core::is_stopword(word)
}

/// Recherche les racines les plus proches dans le dictionnaire (tolérance aux fautes d'orthographe).
#[pyfunction]
#[pyo3(signature = (word, max_distance = 1))]
fn fuzzy_root_lookup(word: &str, max_distance: u32) -> Vec<PyFuzzyMatch> {
    core::fuzzy_root_lookup(word, max_distance)
        .into_iter()
        .map(Into::into)
        .collect()
}

/// Module Python `malagasy_stemmer._core`.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyStemResult>()?;
    m.add_class::<PyFuzzyMatch>()?;
    m.add_class::<PyMalagasyStemmer>()?;
    m.add_function(wrap_pyfunction!(stem, m)?)?;
    m.add_function(wrap_pyfunction!(stem_with_details, m)?)?;
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;
    m.add_function(wrap_pyfunction!(tokenize_and_stem, m)?)?;
    m.add_function(wrap_pyfunction!(tokenize_and_stem_with_details, m)?)?;
    m.add_function(wrap_pyfunction!(is_stopword, m)?)?;
    m.add_function(wrap_pyfunction!(fuzzy_root_lookup, m)?)?;
    Ok(())
}
