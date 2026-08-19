# Référence de l'API Rust

La crate Rust **`malagasy-stemmer`** offre des performances natives extrêmes sans surcoût de runtime (_zero-cost abstractions_), avec zéro allocation superflue et un accès direct au dictionnaire FST embarqué.

---

## Dépendance Cargo

Ajoutez dans votre `Cargo.toml` :

```toml
[dependencies]
malagasy-stemmer = "0.1"
```

---

## Fonctions directes

```rust
use malagasy_stemmer::{stem, tokenize, tokenize_and_stem, is_stopword, fuzzy_root_lookup};

fn main() {
    // 1. Stemming d'un mot isolé
    let root = stem("manoratra");
    assert_eq!(root, "soratra");

    // 2. Tokenisation simple
    let tokens = tokenize("Mianatra teny malagasy isika.");
    assert_eq!(tokens, vec!["mianatra", "teny", "malagasy", "isika"]);

    // 3. Tokenisation et stemming combinés (avec stopwords = true)
    let roots = tokenize_and_stem("Nanoratra boky momba ny fampianarana izy.", true);
    assert_eq!(roots, vec!["soratra", "boky", "anatra"]);

    // 4. Test des stopwords
    assert!(is_stopword("ny"));
    assert!(is_stopword("dia"));
    assert!(!is_stopword("boky"));

    // 5. Recherche floue (Levenshtein)
    let matches = fuzzy_root_lookup("sorata", 1);
    assert_eq!(matches[0].word, "soratra");
    assert_eq!(matches[0].distance, 1);
}
```

---

## La structure `MalagasyStemmer`

Pour traiter des flux de texte ou des lots de tokens :

```rust
use malagasy_stemmer::MalagasyStemmer;

let stemmer = MalagasyStemmer::new();

// Stemming individuel
let res = stemmer.stem_with_details("fampianarana");
println!("Racine : {}, Confiance : {:.2}, Op : {}", res.root, res.confidence, res.operation);

// Traitement par lot (Batch)
let words = vec!["mamaky", "mianatra", "moramora"];
let roots = stemmer.stem_batch(&words);
assert_eq!(roots, vec!["vaky", "anatra", "mora"]);
```

---

## Le dictionnaire FST (`FstDictionary`)

Le dictionnaire de racines est encapsulé dans une structure `FstDictionary` ultra-légère basée sur un transducteur à états finis (`fst::Set`).

```rust
use malagasy_stemmer::default_dictionary;

let dict = default_dictionary();

// Vérification O(k) instantanée (< 15 ns)
assert!(dict.contains("soratra"));
assert!(!dict.contains("manoratra")); // manoratra est un verbe fléchi, pas une racine

// Recherche floue avec un rayon de Levenshtein
let matches = dict.fuzzy_search("vaki", 1);
for m in matches {
    println!("Racine : {}, distance : {}", m.word, m.distance);
}
```

---

## Intégration Tantivy (`features = ["tantivy"]`)

Activez la feature `tantivy` dans votre `Cargo.toml` :

```toml
[dependencies]
tantivy = "0.22"
malagasy-stemmer = { version = "0.1", features = ["tantivy"] }
```

```rust
use malagasy_stemmer::create_malagasy_analyzer;
use tantivy::Index;
use tantivy::schema::*;

// 1. Définir le schéma avec l'analyseur 'malagasy'
let mut schema_builder = Schema::builder();
let text_options = TextOptions::default()
    .set_indexing_options(
        TextFieldIndexing::default()
            .set_tokenizer("malagasy")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions)
    )
    .set_stored();

let body = schema_builder.add_text_field("body", text_options);
let schema = schema_builder.build();

// 2. Créer l'index et enregistrer l'analyseur
let index = Index::create_in_ram(schema);
index.tokenizers().register("malagasy", create_malagasy_analyzer());
```

---

## Benchmarks Criterion

Pour reproduire les benchmarks de performance sur votre machine :

```bash
cargo bench --package malagasy-stemmer
```

Résultats moyens observés (AMD Ryzen / Intel Core i7) :

- `stem_individual` : **~650 ns / mot**
- `stem_batch_1000_words` : **~0.62 ms** (débit > **1 600 000 mots/sec**)
- `fst_lookup` : **< 15 ns**
- Empreinte mémoire globale du dictionnaire : **53 944 octets (~54 Ko)**
