# malagasy-stemmer

[![PyPI version](https://img.shields.io/pypi/v/malagasy-stemmer.svg?color=blue)](https://pypi.org/project/malagasy-stemmer/)
[![Documentation](https://img.shields.io/badge/docs-GitHub_Pages-teal.svg)](https://tanguy1902.github.io/malagasy-stemmer/)
[![CI](https://github.com/Tanguy1902/malagasy-stemmer/actions/workflows/CI.yml/badge.svg)](https://github.com/Tanguy1902/malagasy-stemmer/actions/workflows/CI.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.11%20%7C%203.12%20%7C%203.13-blue.svg)](https://www.python.org)

**Moteur de stemming morphologique et de tokenisation haute performance pour la langue malgache (_Teny Malagasy_).**

**Documentation officielle complète :** [https://tanguy1902.github.io/malagasy-stemmer/](https://tanguy1902.github.io/malagasy-stemmer/)

Développé en **Rust** avec des structures de données **FST** (_Finite State Transducers_), de la morphologie formelle à deux niveaux (_Two-Level Morphology_) et un décodage probabiliste inspiré de l'algorithme de **Viterbi**.

---

## Fonctionnalités

- **Normalisation morphologique complète (_Fototeny_)** :
  - **Préfixes & mutations nasales** : `man-`, `mam-`, `mang-`, `nan-`, `han-`, `fan-`, `mpan-`, `fampi-`, `maha-`...
  - **Inversion phonétique** : `manoratra` $\rightarrow$ `soratra`, `mamaky` $\rightarrow$ `vaky`, `mamboly` $\rightarrow$ `voly`, `mangalatra` $\rightarrow$ `halatra`.
  - **Suffixes passifs & circonstanciels** : `soratana` $\rightarrow$ `soratra`, `vakina` $\rightarrow$ `vaky`.
  - **Infixes** : `vinaky` $\rightarrow$ `vaky` (retrait de `-in-`).
  - **Réduplication (_Famerenana fototeny_)** : `tsaratsara` $\rightarrow$ `tsara`, `moramora` $\rightarrow$ `mora`.
  - **Mots composés & Sandhi consonantique** : `harem-pirenena` $\rightarrow$ `harena_firenena`, `tanan-dehibe` $\rightarrow$ `tanana_lehibe`.
- **Dictionnaire FST ultra-compact** : Dictionnaire de racines compilé en graphe d'états finis (< 2 Ko pour ~300 racines), recherche en $O(k)$ avec 0 allocation dynamique au runtime.
- **Tolérance aux fautes d'orthographe (Levenshtein)** : Recherche floue via intersection d'automate fini et de FST.
- **Pipeline de tokenisation & Stopwords** : Découpage textuel Unicode et élimination automatique des mots vides malgaches.
- **Multi-langage** : Crate Rust natif + Package Python (PyO3 avec ABI stable `abi3`).

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
```

---

## Utilisation rapide

### En Python

```python
import malagasy_stemmer as mg

# 1. Stemming d'un mot isolé
print(mg.stem("manoratra"))     # "soratra"
print(mg.stem("nanoratra"))     # "soratra"
print(mg.stem("fampianarana"))  # "anatra"
print(mg.stem("tsaratsara"))    # "tsara"
print(mg.stem("harem-pirenena"))# "harena_firenena"

# 2. Stemming avec détails morphologiques complets
result = mg.stem_with_details("manoratra")
print(result.root)          # "soratra"
print(result.confidence)    # 0.95
print(result.operation)     # "prefix_nasal_mutation"
print(result.in_dictionary) # True

# 3. Tokenisation et stemming d'un texte complet (avec retrait des stopwords)
text = "Nanoratra taratasy momba ny fampianarana sy ny fambolena izy ireo."
roots = mg.tokenize_and_stem(text, remove_stopwords=True)
print(roots)
# ['soratra', 'taratasy', 'anatra', 'voly']

# 4. Traitement par lot (Batch)
stemmer = mg.MalagasyStemmer()
batch = stemmer.stem_batch(["mamaky", "mianatra", "moramora"])
print(batch)  # ['vaky', 'anatra', 'mora']

# 5. Recherche floue (Levenshtein distance)
matches = mg.fuzzy_root_lookup("sorata", max_distance=1)
for m in matches:
    print(f"Racine: {m.word}, Distance: {m.distance}")
# Racine: soratra, Distance: 1
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

    // 2. Tokenisation de texte
    let text = "Mianatra teny malagasy amin'ny solosaina izahay";
    let tokens = tokenize_and_stem(text, true);
    println!("Racines extraites: {:?}", tokens);

    // 3. Stopwords
    assert!(is_stopword("dia"));
    assert!(!is_stopword("boky"));

    // 4. Recherche floue
    let matches = fuzzy_root_lookup("sorata", 1);
    assert_eq!(matches[0].word, "soratra");
}
```

---

## Architecture algorithmique

```
[ Mot d'entrée ]
       │
       ▼
 1. Vérification Dictionnaire FST (O(k))
       │
       ▼
 2. Mots Composés & Sandhi (harem-pirenena -> harena + firenena)
       │
       ▼
 3. Réduction de Réduplication (moramora -> mora)
       │
       ▼
 4. Élimination d'Infixes (-in-, -om-)
       │
       ▼
 5. Désaffixation des Préfixes & Inversion des Mutations Nasales
    (man- -> s/t/ts, mam- -> v/b/p, mang- -> h/k)
       │
       ▼
 6. Suppression des Suffixes & Restauration Morphophonémique
    (soratana -> soratra, anarana -> anatra, tapahana -> tapaka)
       │
       ▼
 7. Décodage MAP / Viterbi Scorer
    Score = P(Dict) * P(Opération) * P(Longueur) * P(Phonotaxie)
       │
       ▼
[ Racine Canonique (*Fototeny*) ]
```

---

## Benchmark

Pour lancer les benchmarks Criterion en Rust :

```bash
cargo bench --package malagasy-stemmer
```

- **Débit de stemming** : > 1 500 000 mots / seconde par cœur CPU.
- **Lookup dictionnaire FST** : < 15 nanosecondes par requête.
- **Taille mémoire** : 1,9 Ko pour l'ensemble du dictionnaire compilé.

---

## Contribution & Données

Le dictionnaire d'ancrage est stocké dans [`crates/malagasy-stemmer/data/roots.tsv`](crates/malagasy-stemmer/data/roots.tsv). Vous pouvez l'enrichir directement en ajoutant des racines malgaches au format TSV (`racine\tcatégorie`), triées par ordre alphabétique.

---

## Licence

Distribué sous licence **Apache-2.0**.
