//! Exemple complet d'intégration de `malagasy-stemmer` dans le moteur de recherche **Tantivy**.

#[cfg(feature = "tantivy")]
fn main() -> tantivy::Result<()> {
    use malagasy_stemmer::create_malagasy_analyzer;
    use tantivy::collector::TopDocs;
    use tantivy::doc;
    use tantivy::query::QueryParser;
    use tantivy::schema::*;
    use tantivy::{doc, Index, TantivyDocument};

    println!("=== Tantivy + Malagasy Stemmer Search Engine Demo ===\n");

    // 1. Définition du schéma
    let mut schema_builder = Schema::builder();

    let text_field_indexing = TextFieldIndexing::default()
        .set_tokenizer("malagasy")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);

    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();

    let id_field = schema_builder.add_u64_field("id", INDEXED | STORED);
    let title_field = schema_builder.add_text_field("title", STRING | STORED);
    let body_field = schema_builder.add_text_field("body", text_options);
    let schema = schema_builder.build();

    // 2. Création de l'index en mémoire
    let index = Index::create_in_ram(schema);

    // 3. Enregistrement de l'analyseur morphologique malgache
    index
        .tokenizers()
        .register("malagasy", create_malagasy_analyzer());

    // 4. Indexation de documents en malgache
    let mut index_writer = index.writer(15_000_000)?;

    let documents = vec![
        (
            1,
            "Fampianarana ambony",
            "Niresaka momba ny fampianarana sy ny sekoly ambony eto Madagasikara ny minisitera.",
        ),
        (
            2,
            "Fambolena sy Fiompiana",
            "Mamboly vary sy miompy omby amin'ny tanimbary lonaka ny tantsaha any amin'ny faritra.",
        ),
        (
            3,
            "Teknolojia sy Serasera",
            "Mampiasa solosaina sy finday ny tanora amin'ny fikarohana sy ny fianarana.",
        ),
        (
            4,
            "Fiarovana ny harem-pirenena",
            "Zava-dehibe ny fiarovana ny harem-pirenena sy ny ala amin'ny fanimbana.",
        ),
    ];

    for (id, title, body) in &documents {
        index_writer.add_document(doc!(
            id_field => *id,
            title_field => *title,
            body_field => *body
        ))?;
        println!("+ Indexé : [{}] {}", id, title);
    }

    index_writer.commit()?;
    println!("\n-> Indexation terminée ({} documents).\n", documents.len());

    // 5. Recherche avec requêtes contenant des formes fléchies
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![body_field]);

    let test_queries = vec![
        ("mianatra", "Forme verbale pour chercher fampianarana / fianarana"),
        ("soratana", "Passif pour tester le stemming"),
        ("voly", "Racine pure pour chercher mamboly"),
        ("solosainako", "Possessif pour chercher solosaina"),
        ("harena", "Racine pour chercher harem-pirenena"),
    ];

    for (query_str, description) in test_queries {
        println!("--------------------------------------------------");
        println!("--- Requête : \"{}\" ({})", query_str, description);

        let query = query_parser.parse_query(query_str)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(3))?;

        if top_docs.is_empty() {
            println!("   [x] Aucun document trouvé.");
        } else {
            for (score, doc_address) in top_docs {
                let retrieved: TantivyDocument = searcher.doc(doc_address)?;
                let title = retrieved.get_first(title_field).unwrap().as_str().unwrap();
                let id = retrieved.get_first(id_field).unwrap().as_u64().unwrap();
                println!("   [Score: {:.4}] Doc #{} : \"{}\"", score, id, title);
            }
        }
        println!();
    }

    println!("\n=== Fin de la démonstration ===");
    Ok(())
}

#[cfg(not(feature = "tantivy"))]
fn main() {
    println!("Cet exemple nécessite la feature 'tantivy' :");
    println!("cargo run --example tantivy_search --features tantivy");
}
