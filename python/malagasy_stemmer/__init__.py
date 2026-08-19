"""Malagasy Stemmer (malagasy-stemmer).

High-performance morphological stemmer and tokenizer for the Malagasy language (Teny Malagasy),
written in Rust with FST dictionaries and probabilistic Viterbi decoding.

Example usage:
    >>> import malagasy_stemmer as mg
    >>> mg.stem("manoratra")
    'soratra'
    >>> mg.stem("fampianarana")
    'anatra'
    >>> mg.tokenize_and_stem("Nanoratra taratasy momba ny fampianarana izy")
    ['soratra', 'taratasy', 'anatra']
"""

from malagasy_stemmer._core import (
    FuzzyMatch,
    MalagasyStemmer,
    StemResult,
    fuzzy_root_lookup,
    is_stopword,
    stem,
    stem_with_details,
    tokenize,
    tokenize_and_stem,
    tokenize_and_stem_with_details,
)

__version__ = "0.1.0"

__all__ = [
    "FuzzyMatch",
    "MalagasyStemmer",
    "StemResult",
    "fuzzy_root_lookup",
    "is_stopword",
    "stem",
    "stem_with_details",
    "tokenize",
    "tokenize_and_stem",
    "tokenize_and_stem_with_details",
]
