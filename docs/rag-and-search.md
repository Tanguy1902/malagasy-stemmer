# Intégration RAG & Moteurs de Recherche

Le stemming morphologique est le maillon essentiel pour obtenir un **rappel (recall)** élevé lors de la recherche textuelle en langue malgache.

Sans stemming :

- Un document contenant _"fampianarana"_ (enseignement) ne sera **jamais trouvé** si l'utilisateur tape _"mianatra"_ (apprendre) ou _"mpianatra"_ (élève).
- Un document contenant _"fambolena"_ (agriculture) sera ignoré pour une requête _"mamboly"_ (cultiver).

Avec `malagasy-stemmer`, tous ces termes convergent vers leur racine canonique : **`anatra`** et **`voly`**.

---

## 1. Intégration avec BM25 (Python `rank_bm25`)

```python
from rank_bm25 import BM25Okapi
import malagasy_stemmer as mg

# 1. Corpus de documents bruts en malgache
corpus = [
    "Ny tantaran'ny fampianarana eto Madagasikara dia nisy fivoarana lehibe.",
    "Zava-dehibe ny fambolena vary sy ny fiompiana omby amin'ny toekarena.",
    "Nanoratra taratasy momba ny fiarovana ny harem-pirenena ny mpikaroka.",
]

# 2. Tokenisation & Stemming des documents
tokenized_corpus = [
    mg.tokenize_and_stem(doc, remove_stopwords=True)
    for doc in corpus
]

bm25 = BM25Okapi(tokenized_corpus)

# 3. Requête utilisateur (formes fléchies différentes)
query = "Te hianatra momba ny fambolena aho"
tokenized_query = mg.tokenize_and_stem(query, remove_stopwords=True)
# tokenized_query -> ['anatra', 'voly']

# 4. Scoring et récupération des documents les plus pertinents
scores = bm25.get_scores(tokenized_query)
best_doc_idx = scores.argmax()

print(f"Meilleur document trouvé : {corpus[best_doc_idx]}")
# Trouve immédiatement le document 1 et 2 grâce aux racines 'anatra' et 'voly' !
```

---

## 2. Intégration dans un pipeline LangChain / LlamaIndex

Vous pouvez utiliser `malagasy-stemmer` comme préprocesseur ou fonction de transformation pour vos index hybrides :

```python
import malagasy_stemmer as mg

def malagasy_preprocessor(text: str) -> str:
    """Transforme un texte malgache en une séquence de racines canoniques."""
    roots = mg.tokenize_and_stem(text, remove_stopwords=True)
    return " ".join(roots)

# Exemple d'utilisation
raw_chunk = "Niresaka momba ny fampandrosoana ny fambolena sy ny fampianarana izy ireo."
normalized_chunk = malagasy_preprocessor(raw_chunk)
print(normalized_chunk)
# "resaka roso voly anatra"
```

---

## 3. Intégration dans Tantivy / Meilisearch (Rust)

Dans les moteurs de recherche écrits en Rust comme **Tantivy**, vous pouvez implémenter le trait `TokenFilter` avec `malagasy_stemmer::stem` pour créer un filtre de recherche malgache natif ultra-rapide.

```rust
use malagasy_stemmer::stem;

// Pseudo-code d'un filtre Tantivy / Search Engine
fn stem_token_stream(tokens: Vec<String>) -> Vec<String> {
    tokens.into_iter().map(|t| stem(&t)).collect()
}
```
