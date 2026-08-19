#[cfg(feature = "tantivy")]
mod tantivy_tests {
    use malagasy_stemmer::create_malagasy_analyzer;
    use tantivy::collector::TopDocs;
    use tantivy::doc;
    use tantivy::query::QueryParser;
    use tantivy::schema::*;
    use tantivy::{Index, TantivyDocument};

    #[test]
    fn test_tantivy_malagasy_stem_search() -> tantivy::Result<()> {
        let mut schema_builder = Schema::builder();

        let text_field_indexing = TextFieldIndexing::default()
            .set_tokenizer("malagasy")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);

        let text_options = TextOptions::default()
            .set_indexing_options(text_field_indexing)
            .set_stored();

        let title = schema_builder.add_text_field("title", STRING | STORED);
        let body = schema_builder.add_text_field("body", text_options);
        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema.clone());

        // Enregistrer l'analyseur morphologique malgache
        index
            .tokenizers()
            .register("malagasy", create_malagasy_analyzer());

        let mut index_writer = index.writer(15_000_000)?;

        // Doc 1 : Contient "Nanoratra" et "fampianarana"
        index_writer.add_document(doc!(
            title => "Doc 1",
            body => "Nanoratra taratasy momba ny fampianarana sy ny fambolena ny mpianatra."
        ))?;

        // Doc 2 : Contient "Mamboly" et "omby"
        index_writer.add_document(doc!(
            title => "Doc 2",
            body => "Mamboly vary sy miompy omby amin'ny tanimbary ny tantsaha."
        ))?;

        // Doc 3 : Contient "harem-pirenena"
        index_writer.add_document(doc!(
            title => "Doc 3",
            body => "Fiarovana ny harem-pirenena sy ny tontolo iainana eto Madagasikara."
        ))?;

        index_writer.commit()?;

        let reader = index.reader()?;
        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, vec![body]);

        // Test 1: Chercher "soratana" (passif) doit trouver Doc 1 (qui contient "Nanoratra" actif passé)
        let query1 = query_parser.parse_query("soratana")?;
        let top_docs1 = searcher.search(&query1, &TopDocs::with_limit(5))?;
        assert_eq!(top_docs1.len(), 1);
        let retrieved_doc1: TantivyDocument = searcher.doc(top_docs1[0].1)?;
        assert_eq!(
            retrieved_doc1.get_first(title).unwrap().as_str().unwrap(),
            "Doc 1"
        );

        // Test 2: Chercher "mianatra" (verbe) doit trouver Doc 1 (qui contient "fampianarana" et "mpianatra")
        let query2 = query_parser.parse_query("mianatra")?;
        let top_docs2 = searcher.search(&query2, &TopDocs::with_limit(5))?;
        assert_eq!(top_docs2.len(), 1);

        // Test 3: Chercher "fambolena" (circonfixe) doit trouver Doc 1 et Doc 2 (qui contient "Mamboly")
        let query3 = query_parser.parse_query("fambolena")?;
        let top_docs3 = searcher.search(&query3, &TopDocs::with_limit(5))?;
        assert_eq!(top_docs3.len(), 2);

        // Test 4: Chercher "harena" doit trouver Doc 3 (qui contient le composé "harem-pirenena")
        let query4 = query_parser.parse_query("harena")?;
        let top_docs4 = searcher.search(&query4, &TopDocs::with_limit(5))?;
        assert_eq!(top_docs4.len(), 1);

        Ok(())
    }
}
