use tantivy::tokenizer::{LowerCaser, RemoveLongFilter, SimpleTokenizer, TextAnalyzer, Token, TokenFilter, TokenStream, Tokenizer};
use crate::stem;

/// Filtre de tokens Tantivy pour le stemming morphologique en langue malgache.
/// Intègre `malagasy-stemmer` directement dans le pipeline d'indexation et de recherche de Tantivy.

#[derive(Clone, Copy, Debug, Default)]
pub struct MalagasyStemFilter;

#[derive(Clone, Debug)]
pub struct MalagasyStemFilterWrapper<T> {
    underlying: T,
}

impl<T: Tokenizer> Tokenizer for MalagasyStemFilterWrapper<T> {
    type TokenStream<'a> = MalagasyStemTokenStream<T::TokenStream<'a>>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        MalagasyStemTokenStream {
            tail: self.underlying.token_stream(text),
        }
    }
}

impl TokenFilter for MalagasyStemFilter {
    type Tokenizer<T: Tokenizer> = MalagasyStemFilterWrapper<T>;

    fn transform<T: Tokenizer>(self, tokenizer: T) -> Self::Tokenizer<T> {
        MalagasyStemFilterWrapper {
            underlying: tokenizer,
        }
    }
}

/// Flux de tokens transformé appliquant le stemming malgache à chaque token.
pub struct MalagasyStemTokenStream<T> {
    tail: T,
}

impl<T: TokenStream> TokenStream for MalagasyStemTokenStream<T> {
    fn advance(&mut self) -> bool {
        if self.tail.advance() {
            let token = self.tail.token_mut();
            let stemmed = stem(&token.text);
            token.text = stemmed;
            true
        } else {
            false
        }
    }

    fn token(&self) -> &Token {
        self.tail.token()
    }

    fn token_mut(&mut self) -> &mut Token {
        self.tail.token_mut()
    }
}

/// Crée un analyseur de texte Tantivy pré-configuré complet pour la langue malgache.
///
/// Pipeline :
/// 1. `SimpleTokenizer` : Découpage Unicode des mots.
/// 2. `RemoveLongFilter(40)` : Élimination des tokens anormalement longs.
/// 3. `LowerCaser` : Normalisation en minuscules.
/// 4. `MalagasyStemFilter` : Réduction morphologique vers la racine (*fototeny*).
///
/// # Exemple
/// ```rust
/// use malagasy_stemmer::tantivy_integration::create_malagasy_analyzer;
///
/// let mut analyzer = create_malagasy_analyzer();
/// let mut stream = analyzer.token_stream("Mamboly vary ny tantsaha");
///
/// let mut tokens = Vec::new();
/// while stream.advance() {
///     tokens.push(stream.token().text.clone());
/// }
/// assert_eq!(tokens, vec!["voly", "vary", "ny", "tantsaha"]);
/// ```
pub fn create_malagasy_analyzer() -> TextAnalyzer {
    TextAnalyzer::builder(SimpleTokenizer::default())
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .filter(MalagasyStemFilter)
        .build()
}
