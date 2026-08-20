# malagasy-stemmer

<p align="center">
  <strong>Moteur de stemming morphologique, tokenizer et filtre de recherche haute performance pour la langue malgache (<em>Teny Malagasy</em>).</strong>
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
  <a href="README.md"><strong>[ English Documentation ]</strong></a>
</p>

---

## Présentation

**`malagasy-stemmer`** est une bibliothèque logicielle native développée en **Rust** avec des bindings **Python**, conçue pour combler un vide technologique critique dans le traitement automatique du langage naturel (**NLP**) et la recherche textuelle (**IR**) pour la langue malgache (*Teny Malagasy*, langue austronésienne parlée par plus de 30 millions de personnes).

Le malgache possède une morphologie agglutinante particulièrement riche (mutations consonantiques nasales, alternances morphophonémiques, infixes et Sandhi consonantique dans les mots composés). `malagasy-stemmer` permet de réduire fidèlement toute forme fléchie vers sa racine canonique (*fototeny*).

---

## Fonctionnalités Clés

- **Normalisation morphologique complète (*Fototeny*)** :
  - **Mutations nasales** : `manoratra` $\rightarrow$ `soratra`, `mamaky` $\rightarrow$ `vaky`, `mamboly` $\rightarrow$ `voly`, `mangalatra` $\rightarrow$ `halatra`.
  - **Suffixes passifs & impératifs** : `soratana` $\rightarrow$ `soratra`, `vakina` $\rightarrow$ `vaky`, `tapaho` $\rightarrow$ `tapaka`.
  - **Infixes aspectuels** : `vinaky` $\rightarrow$ `vaky` (infixe `-in-`), `tinapaka` $\rightarrow$ `tapaka`.
  - **Réduplications** : `tsaratsara` $\rightarrow$ `tsara`, `moramora` $\rightarrow$ `mora`, `fotsifotsy` $\rightarrow$ `fotsy`.
  - **Mots composés & Sandhi** : `harem-pirenena` $\rightarrow$ `harena_firenena`, `tanan-dehibe` $\rightarrow$ `tanana_lehibe`, `ara-politika` $\rightarrow$ `araka_politika`.
  - **Emprunts modernes malgachisés (*Teny nohagasiana*)** : `governemanta`, `politika`, `demokrasia`, `solosaina`, `telefaonina`, `banky`.
- **Dictionnaire FST ultra-compact** : **9 960+ racines canoniques pures** compilées en un graphe d'états finis (~54 Ko en mémoire, lookup $O(k) < 15$ ns, 0 allocation dynamique).
- **Intégration Moteur de Recherche (Filtre Tantivy natif)** : `MalagasyStemFilter` et `create_malagasy_analyzer()` intégrés pour Tantivy.
- **Évaluation formelle standardisée** : **74.91% d'Exact Match** sur 1 387 paires de test de référence (+58% vs baseline naïve).
- **Vitesse extrême** : **> 1 500 000 mots/seconde** par cœur CPU.
- **Tolérance aux fautes d'orthographe** : Recherche floue de racines via automates de Levenshtein.
- **Tokenisation & Stopwords** : Découpage Unicode, gestion des clitiques (`amin'ny`, `an'i`), et filtrage automatique des mots vides malgaches.

---

## Résultats des Benchmarks

Évaluation sur **1 387 paires morphologiques étiquetées** issues du *Rakibolana Malagasy* et de règles canoniques :

| Catégorie Morphologique | Échantillons | `malagasy-stemmer` | Baseline Naïve | Distance Levenshtein Moyenne |
| :--- | :---: | :---: | :---: | :---: |
| `irregular_suppletive` | 39 | **100.00%** | 12.82% | 0.00 |
| `reduplication` | 21 | **100.00%** | 0.00% | 0.00 |
| `infix` | 110 | **91.82%** | 0.00% | 0.11 |
| `simple_prefix` | 300 | **80.33%** | 63.33% | 0.58 |
| `nasal_active` | 300 | **72.67%** | 10.67% | 0.53 |
| `passive_suffix` | 300 | **72.33%** | 0.00% | 0.85 |
| `circumfix` | 300 | **64.67%** | 2.33% | 1.21 |
| `compounds_sandhi` | 17 | **47.06%** | 0.00% | 1.76 |
| **SCORE GLOBAL** | **1 387** | **74.91%** | **16.87%** | **0.72** |

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

# Avec support du moteur de recherche Tantivy :
# malagasy-stemmer = { version = "0.1", features = ["tantivy"] }
```

---

## Utilisation rapide

### En Python

```python
import malagasy_stemmer as mg

# 1. Stemming d'un mot isolé
print(mg.stem("manoratra"))      # -> "soratra"
print(mg.stem("fampianarana"))   # -> "anatra"
print(mg.stem("harem-pirenena")) # -> "harena_firenena"
print(mg.stem("ara-politika"))   # -> "araka_politika"

# 2. Métadonnées morphologiques détaillées
res = mg.stem_with_details("manoratra")
print(res.root)          # "soratra"
print(res.confidence)    # 1.0
print(res.operation)     # "prefix_nasal_mutation"
print(res.in_dictionary) # True

# 3. Tokenisation et stemming de texte avec retrait des stopwords
text = "Niresaka tamin'ny mpianatra momba ny fampianarana sy ny fambolena izy ireo."
roots = mg.tokenize_and_stem(text, remove_stopwords=True)
print(roots)
# ['resaka', 'anatra', 'anatra', 'voly']

# 4. Traitement par lot (Batch)
stemmer = mg.MalagasyStemmer()
print(stemmer.stem_batch(["mamaky", "mianatra", "moramora"]))
# ['vaky', 'anatra', 'mora']

# 5. Recherche floue (Distance de Levenshtein)
matches = mg.fuzzy_root_lookup("sorata", max_distance=1)
for m in matches:
    print(f"Racine trouvée : {m.word} (distance: {m.distance})")
# Racine trouvée : soratra (distance: 1)
```

---

### En Rust

```rust
use malagasy_stemmer::{stem, tokenize_and_stem, is_stopword, fuzzy_root_lookup};

fn main() {
    // 1. Stemming simple
    assert_eq!(stem("manoratra"), "soratra");
    assert_eq!(stem("fampianarana"), "anatra");
    assert_eq!(stem("harem-pirenena"), "harena_firenena");

    // 2. Tokenisation et stemming avec retrait des stopwords
    let text = "Nanoratra taratasy momba ny fampianarana ny mpianatra.";
    let tokens = tokenize_and_stem(text, true);
    assert_eq!(tokens, vec!["soratra", "taratasy", "anatra", "anatra"]);

    // 3. Vérification des stopwords
    assert!(is_stopword("ny"));
    assert!(!is_stopword("boky"));

    // 4. Recherche floue
    let matches = fuzzy_root_lookup("sorata", 1);
    assert_eq!(matches[0].word, "soratra");
}
```

---

### Intégration Tantivy (Rust)

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
    
    // Enregistrement de l'analyseur morphologique malgache
    index.tokenizers().register("malagasy", create_malagasy_analyzer());
    
    // Une recherche sur "soratana" trouvera immédiatement les documents indexés avec "nanoratra" !
    Ok(())
}
```

---

## Documentation officielle

Documentation complète : **[https://tanguy1902.github.io/malagasy-stemmer/](https://tanguy1902.github.io/malagasy-stemmer/)**

- [Guide de démarrage](https://tanguy1902.github.io/malagasy-stemmer/getting-started/)
- [Théorie linguistique & Règles morphologiques](https://tanguy1902.github.io/malagasy-stemmer/linguistics/)
- [Référence API Python](https://tanguy1902.github.io/malagasy-stemmer/python-api/)
- [Référence API Rust](https://tanguy1902.github.io/malagasy-stemmer/rust-api/)
- [Intégration RAG & Moteurs de recherche](https://tanguy1902.github.io/malagasy-stemmer/rag-and-search/)
- [Rapport d'évaluation formelle](https://tanguy1902.github.io/malagasy-stemmer/benchmark-results/)

---

## Licence

Distribué sous licence **Apache-2.0**. Voir [LICENSE](LICENSE) pour plus de détails.
