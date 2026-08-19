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

## 3. Intégration dans le moteur de recherche Tantivy (Rust)

`malagasy-stemmer` s'intègre nativement dans le moteur de recherche full-text **Tantivy** via la feature optionnelle `tantivy` (`features = ["tantivy"]`).

Elle fournit `MalagasyStemFilter` (implémentant `tantivy::tokenizer::TokenFilter`) ainsi qu'un analyseur prêt à l'emploi `create_malagasy_analyzer()`.

### Configuration Cargo (`Cargo.toml`)

```toml
[dependencies]
tantivy = "0.22"
malagasy-stemmer = { version = "0.1", features = ["tantivy"] }
```

### Exemple complet d'indexation et de recherche

```rust
use malagasy_stemmer::create_malagasy_analyzer;
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, TantivyDocument};

fn main() -> tantivy::Result<()> {
    // 1. Définition du schéma avec analyseur 'malagasy'
    let mut schema_builder = Schema::builder();
    let text_options = TextOptions::default()
        .set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("malagasy")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        )
        .set_stored();

    let title = schema_builder.add_text_field("title", STRING | STORED);
    let body = schema_builder.add_text_field("body", text_options);
    let schema = schema_builder.build();

    let index = Index::create_in_ram(schema);

    // 2. Enregistrer l'analyseur morphologique
    index
        .tokenizers()
        .register("malagasy", create_malagasy_analyzer());

    // 3. Indexer des documents
    let mut writer = index.writer(15_000_000)?;
    writer.add_document(doc!(
        title => "Fampianarana",
        body => "Nanoratra taratasy momba ny fampianarana ny mpianatra."
    ))?;
    writer.commit()?;

    // 4. Recherche avec une forme fléchie différente ("soratana" ou "mianatra")
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![body]);

    // La requête "soratana" (passif) matche "Nanoratra" (actif passé) grâce à la racine 'soratra' !
    let query = query_parser.parse_query("soratana")?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(5))?;

    for (_score, doc_address) in top_docs {
        let retrieved: TantivyDocument = searcher.doc(doc_address)?;
        println!("Document trouvé : {:?}", retrieved.get_first(title).unwrap().as_str());
    }

    Ok(())
}
```
