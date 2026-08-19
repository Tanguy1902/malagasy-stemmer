# Guide de démarrage

Ce guide vous accompagne pas à pas pour installer et utiliser **`malagasy-stemmer`** dans vos projets en **Python** ou en **Rust**.

---

## 🐍 Installation en Python

`malagasy-stemmer` est distribué sous forme de *wheels* binaires précompilées sur **PyPI** pour Linux (x86_64, aarch64), macOS (Intel, Apple Silicon) et Windows. Aucun compilateur C/Rust n'est requis sur la machine cible.

### Prérequis
- Python $\ge$ 3.11

### Commande d'installation
```bash
pip install malagasy-stemmer
```

### Vérifier l'installation
```bash
python -c "import malagasy_stemmer as mg; print('Version OK, test:', mg.stem('manoratra'))"
```
Sortie attendue :
```text
Version OK, test: soratra
```

---

## 🦀 Installation en Rust

Pour intégrer le moteur natif dans un projet Rust :

### `Cargo.toml`
```toml
[dependencies]
malagasy-stemmer = "0.1"
```

### Exemple minimal (`main.rs`)
```rust
use malagasy_stemmer::{stem, is_stopword};

fn main() {
    let word = "fampianarana";
    let root = stem(word);
    println!("Mot : {} -> Racine : {}", word, root); // anatra

    println!("'ny' est un mot vide ? {}", is_stopword("ny")); // true
}
```

---

## 🚀 Premiers cas d'usage

### 1. Normaliser des mots isolés
```python
import malagasy_stemmer as mg

# Verbes à tous les temps
print(mg.stem("manoratra"))  # Présent -> soratra
print(mg.stem("nanoratra"))  # Passé -> soratra
print(mg.stem("hanoratra"))  # Futur -> soratra

# Dérivés nominaux et passifs
print(mg.stem("mpanoratra"))   # Nom d'agent -> soratra
print(mg.stem("fanoratana"))   # Nom circonstanciel -> soratra
print(mg.stem("soratana"))     # Passif -> soratra
```

### 2. Traiter un paragraphe complet
```python
text = """
Niresaka tamin'ny mpianatra momba ny fampianarana sy ny 
fambolena eto Madagasikara izy ireo omaly.
"""

# Découpe le texte, retire la ponctuation et les stopwords, et extrait chaque racine :
roots = mg.tokenize_and_stem(text, remove_stopwords=True)
print(roots)
# ['resaka', 'anatra', 'anatra', 'voly', 'madagasikara']
```

### 3. Traitement par lot (Batch processing)
Pour les gros volumes de données (milliers de documents), utilisez la classe `MalagasyStemmer` avec `stem_batch` pour un débit maximal :

```python
stemmer = mg.MalagasyStemmer()

vocab = ["mamaky", "mianatra", "moramora", "harem-pirenena", "fandrosoana"]
roots = stemmer.stem_batch(vocab)
print(roots)
# ['vaky', 'anatra', 'mora', 'harena_firenena', 'roso']
```
