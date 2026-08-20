# malagasy-stemmer

<p align="center">
  <strong>High-performance morphological stemmer, tokenizer, and search filter for the Malagasy language (<em>Teny Malagasy</em>).</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/malagasy-stemmer"><img src="https://img.shields.io/crates/v/malagasy-stemmer.svg?color=orange" alt="crates.io"></a>
  <a href="https://pypi.org/project/malagasy-stemmer/"><img src="https://img.shields.io/pypi/v/malagasy-stemmer.svg?color=blue" alt="PyPI version"></a>
  <a href="https://tanguy1902.github.io/malagasy-stemmer/"><img src="https://img.shields.io/badge/docs-GitHub_Pages-teal.svg" alt="Documentation"></a>
  <a href="https://github.com/Tanguy1902/malagasy-stemmer/actions/workflows/CI.yml"><img src="https://github.com/Tanguy1902/malagasy-stemmer/actions/workflows/CI.yml/badge.svg" alt="CI Status"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" alt="License"></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust 2021"></a>
  <a href="https://www.python.org"><img src="https://img.shields.io/badge/python-3.11%20%7C%203.12%20%7C%203.13-blue.svg" alt="Python Versions"></a>
</p>

<p align="center">
  <a href="README_fr.md"><strong>[ Documentation en Français ]</strong></a>
</p>

---

## Overview

**`malagasy-stemmer`** is a native Rust library with Python bindings designed to bridge a critical gap in Natural Language Processing (**NLP**) and Information Retrieval (**IR**) for the Malagasy language (*Teny Malagasy*, an Austronesian language spoken by 30+ million people).

Malagasy features a rich, agglutinative morphology with complex nasal mutations, infixation, reduplication, and consonant sandhi in compound words. `malagasy-stemmer` accurately reduces inflected surface forms to their canonical roots (*fototeny*).

---

## Key Features

- **Accurate Morphological Normalization (*Fototeny*)**:
  - **Nasal mutations**: `manoratra` $\rightarrow$ `soratra`, `mamaky` $\rightarrow$ `vaky`, `mamboly` $\rightarrow$ `voly`, `mangalatra` $\rightarrow$ `halatra`.
  - **Passive & imperative suffixes**: `soratana` $\rightarrow$ `soratra`, `vakina` $\rightarrow$ `vaky`, `tapaho` $\rightarrow$ `tapaka`.
  - **Infixes**: `vinaky` $\rightarrow$ `vaky` (infix `-in-`), `tinapaka` $\rightarrow$ `tapaka`.
  - **Reduplication**: `tsaratsara` $\rightarrow$ `tsara`, `moramora` $\rightarrow$ `mora`, `fotsifotsy` $\rightarrow$ `fotsy`.
  - **Compound words & Consonant Sandhi**: `harem-pirenena` $\rightarrow$ `harena_firenena`, `tanan-dehibe` $\rightarrow$ `tanana_lehibe`, `ara-politika` $\rightarrow$ `araka_politika`.
  - **Modern Malagasy Loanwords (*Teny nohagasiana*)**: `governemanta`, `politika`, `demokrasia`, `solosaina`, `telefaonina`, `banky`.
- **Ultra-compact FST Dictionary**: **9,960+ pure canonical roots** compiled into a Finite State Transducer graph (~54 KB in memory, $O(k)$ lookup $< 15$ ns, zero dynamic allocations).
- **Search Engine Ready (Tantivy Native TokenFilter)**: Built-in `MalagasyStemFilter` and `create_malagasy_analyzer()` for the Tantivy search engine.
- **Formally Evaluated**: **74.91% Exact Match** across 1,387 gold-standard test pairs (+58% absolute gain vs. naive rule-based baselines).
- **Extreme Speed**: **> 1,500,000 words/second** per CPU core.
- **Typo Tolerance**: Fast fuzzy root lookup via Levenshtein automata.
- **Tokenization & Stopwords**: Smart Unicode punctuation splitter, clitic handling (`amin'ny`, `an'i`), and automatic Malagasy stopword filtering.

---

## Benchmark & Accuracy

Evaluated on **1,387 labeled test pairs** spanning 8 morphological dimensions:

| Morphological Category | Test Samples | `malagasy-stemmer` | Naive Baseline | Avg Levenshtein Distance |
| :--- | :---: | :---: | :---: | :---: |
| `irregular_suppletive` | 39 | **100.00%** | 12.82% | 0.00 |
| `reduplication` | 21 | **100.00%** | 0.00% | 0.00 |
| `infix` | 110 | **91.82%** | 0.00% | 0.11 |
| `simple_prefix` | 300 | **80.33%** | 63.33% | 0.58 |
| `nasal_active` | 300 | **72.67%** | 10.67% | 0.53 |
| `passive_suffix` | 300 | **72.33%** | 0.00% | 0.85 |
| `circumfix` | 300 | **64.67%** | 2.33% | 1.21 |
| `compounds_sandhi` | 17 | **47.06%** | 0.00% | 1.76 |
| **GLOBAL SCORE** | **1,387** | **74.91%** | **16.87%** | **0.72** |

---

## Installation

### Python

```bash
pip install malagasy-stemmer
```

### Rust (`Cargo.toml`)

```toml
[dependencies]
malagasy-stemmer = "0.1"

# With Tantivy search engine support:
# malagasy-stemmer = { version = "0.1", features = ["tantivy"] }
```

---

## Quickstart

### Python

```python
import malagasy_stemmer as mg

# 1. Single word stemming
print(mg.stem("manoratra"))      # -> "soratra"
print(mg.stem("fampianarana"))   # -> "anatra"
print(mg.stem("harem-pirenena")) # -> "harena_firenena"
print(mg.stem("ara-politika"))   # -> "araka_politika"

# 2. Detailed morphological metadata
res = mg.stem_with_details("manoratra")
print(res.root)          # "soratra"
print(res.confidence)    # 1.0
print(res.operation)     # "prefix_nasal_mutation"
print(res.in_dictionary) # True

# 3. Full text tokenization & stemming with stopword removal
text = "Niresaka tamin'ny mpianatra momba ny fampianarana sy ny fambolena izy ireo."
roots = mg.tokenize_and_stem(text, remove_stopwords=True)
print(roots)
# ['resaka', 'anatra', 'anatra', 'voly']

# 4. Batch processing
stemmer = mg.MalagasyStemmer()
print(stemmer.stem_batch(["mamaky", "mianatra", "moramora"]))
# ['vaky', 'anatra', 'mora']

# 5. Fuzzy root lookup (Levenshtein distance)
matches = mg.fuzzy_root_lookup("sorata", max_distance=1)
for m in matches:
    print(f"Match: {m.word} (distance: {m.distance})")
# Match: soratra (distance: 1)
```

---

### Rust

```rust
use malagasy_stemmer::{stem, tokenize_and_stem, is_stopword, fuzzy_root_lookup};

fn main() {
    // 1. Single word stemming
    assert_eq!(stem("manoratra"), "soratra");
    assert_eq!(stem("fampianarana"), "anatra");
    assert_eq!(stem("harem-pirenena"), "harena_firenena");

    // 2. Tokenize and stem with stopword removal
    let text = "Nanoratra taratasy momba ny fampianarana ny mpianatra.";
    let tokens = tokenize_and_stem(text, true);
    assert_eq!(tokens, vec!["soratra", "taratasy", "anatra", "anatra"]);

    // 3. Stopwords check
    assert!(is_stopword("ny"));
    assert!(!is_stopword("boky"));

    // 4. Fuzzy search
    let matches = fuzzy_root_lookup("sorata", 1);
    assert_eq!(matches[0].word, "soratra");
}
```

---

### Tantivy Search Engine Integration (Rust)

```rust
use malagasy_stemmer::create_malagasy_analyzer;
use tantivy::Index;
use tantivy::schema::*;

fn main() -> tantivy::Result<()> {
    let mut schema_builder = Schema::builder();
    let text_options = TextOptions::default()
        .set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("malagasy")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        )
        .set_stored();

    let body = schema_builder.add_text_field("body", text_options);
    let schema = schema_builder.build();

    let index = Index::create_in_ram(schema);
    
    // Register the Malagasy morphological analyzer
    index.tokenizers().register("malagasy", create_malagasy_analyzer());
    
    // Queries like "soratana" will seamlessly match documents with "nanoratra"!
    Ok(())
}
```

---

## Architecture

```
[ Input Word ]
       │
       ▼
 1. Irregular / Suppletive Lookup (mandeha -> leha, homana -> hano)
       │
       ▼
 2. Compound Words & Consonant Sandhi (harem-pirenena -> harena + firenena)
       │
       ▼
 3. Reduplication Reduction (moramora -> mora, fotsifotsy -> fotsy)
       │
       ▼
 4. FST Dictionary Exact Check (O(k) < 15 ns)
       │
       ▼
 5. Infix Removal (-in-, -om-)
       │
       ▼
 6. Prefix Deaffixation & Nasal Mutation Inversion
    (man- -> s/t/ts/h/z, mam- -> v/p/b/f, mang- -> h/k/g)
       │
       ▼
 7. Suffix Deaffixation & Morphophonemic Restoration
    (soratana -> soratra, anarana -> anatra, tapahana -> tapaka, vakina -> vaky)
       │
       ▼
 8. MAP / Viterbi Scorer (P(Dict) * P(Op) * P(Length) * P(Phonotactics))
       │
       ▼
[ Canonical Root (*Fototeny*) ]
```

---

## Documentation

Full official documentation is available at: **[https://tanguy1902.github.io/malagasy-stemmer/](https://tanguy1902.github.io/malagasy-stemmer/)**

- [Getting Started Guide](https://tanguy1902.github.io/malagasy-stemmer/getting-started/)
- [Linguistic Theory & Rules](https://tanguy1902.github.io/malagasy-stemmer/linguistics/)
- [Python API Reference](https://tanguy1902.github.io/malagasy-stemmer/python-api/)
- [Rust API Reference](https://tanguy1902.github.io/malagasy-stemmer/rust-api/)
- [RAG & Search Engine Integration](https://tanguy1902.github.io/malagasy-stemmer/rag-and-search/)
- [Evaluation & Benchmark Results](https://tanguy1902.github.io/malagasy-stemmer/benchmark-results/)

---

## License

Distributed under the **Apache-2.0** License. See [LICENSE](LICENSE) for details.
