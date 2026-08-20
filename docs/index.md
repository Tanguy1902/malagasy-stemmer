# malagasy-stemmer

<p align="center">
  <strong>Moteur de stemming morphologique et de tokenisation haute performance pour la langue malgache (<em>Teny Malagasy</em>).</strong>
</p>

<p align="center">
  <a href="https://pypi.org/project/malagasy-stemmer/"><img src="https://img.shields.io/pypi/v/malagasy-stemmer.svg?color=blue" alt="PyPI version"></a>
  <a href="https://pypi.org/project/malagasy-stemmer/"><img src="https://img.shields.io/pypi/pyversions/malagasy-stemmer.svg" alt="Python Versions"></a>
  <a href="https://github.com/Tanguy1902/malagasy-stemmer/actions/workflows/CI.yml"><img src="https://github.com/Tanguy1902/malagasy-stemmer/actions/workflows/CI.yml/badge.svg" alt="CI Status"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" alt="License"></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust Edition"></a>
</p>

---

## Présentation

**`malagasy-stemmer`** est une bibliothèque logicielle native développée en **Rust** avec des bindings **Python**, conçue pour combler un vide technologique critique dans le traitement automatique du langage naturel (**NLP**) pour les langues peu dotées (_low-resource languages_).

La langue malgache (_Teny Malagasy_, langue austronésienne) possède une morphologie agglutinante particulièrement riche et subtile, caractérisée par des mutations consonantiques nasales, des alternances morphophonémiques, des infixes et du Sandhi consonantique dans les mots composés.

`malagasy-stemmer` résout ces défis en combinant :

- :fontawesome-solid-bolt: **Transducteurs à États Finis (FST)** : Un dictionnaire compact de **plus de 10 000 racines** compilé en graphe d'états finis (< 54 Ko en mémoire, lookup en $O(k)$ < 15 ns).
- **Morphologie formelle à deux niveaux** : Détection et désaffixation récursive des préfixes, suffixes, infixes, réduplications et sandhi.
- **Désambiguïsation probabiliste (Viterbi)** : Sélection du meilleur candidat basée sur la phonotaxie malgache, les priors lexicaux et les pénalités d'affixation.
- **Tolérance aux fautes d'orthographe (Levenshtein)** : Recherche floue de racines ultra-rapide par intersection d'automates.

---

## Exemple en 1 minute

=== "Python"

    ```python
    import malagasy_stemmer as mg

    # 1. Extraction simple de racine (Fototeny)
    print(mg.stem("manoratra"))     # -> 'soratra'
    print(mg.stem("fampianarana"))  # -> 'anatra'
    print(mg.stem("harem-pirenena"))# -> 'harena_firenena'
    print(mg.stem("tsaratsara"))    # -> 'tsara'

    # 2. Tokenisation de texte complet avec élimination des mots vides (Stopwords)
    text = "Nanoratra taratasy momba ny fampianarana sy ny fambolena ny mpianatra."
    tokens = mg.tokenize_and_stem(text, remove_stopwords=True)
    print(tokens)
    # -> ['soratra', 'taratasy', 'anatra', 'voly', 'anatra']
    ```

=== "Rust"

    ```rust
    use malagasy_stemmer::{stem, tokenize_and_stem};

    fn main() {
        assert_eq!(stem("manoratra"), "soratra");
        assert_eq!(stem("fampianarana"), "anatra");
        assert_eq!(stem("harem-pirenena"), "harena_firenena");

        let text = "Nanoratra taratasy momba ny fampianarana ny mpianatra.";
        let roots = tokenize_and_stem(text, true);
        println!("Racines : {:?}", roots);
    }
    ```

---

## Fonctionnalités Clés

| Fonctionnalité             | Description                                                                                                                                                 |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Mutations nasales**      | Rétablissement des consonnes sous-jacentes (`man-` $\rightarrow$ `s/t/ts`, `mam-` $\rightarrow$ `v/b/p`, `mang-` $\rightarrow$ `h/k`).                      |
| **Suffixes & Alternances** | Rétablissement régulier et morphophonémique (`soratana` $\rightarrow$ `soratra`, `vakina` $\rightarrow$ `vaky`, `tapahana` $\rightarrow$ `tapaka`).         |
| **Infixes aspectuels**     | Suppression des infixes `-in-` (passif) et `-om-` (statif) : `vinaky` $\rightarrow$ `vaky`, `tinapaka` $\rightarrow$ `tapaka`.                              |
| **Mots composés & Sandhi** | Séparation et rétablissement des mutations consonantiques : `harem-pirenena` $\rightarrow$ `harena_firenena`, `tanan-dehibe` $\rightarrow$ `tanana_lehibe`. |
| **Réduplication**          | Détection du redoublement expressif ou atténuatif : `moramora` $\rightarrow$ `mora`, `tsaratsara` $\rightarrow$ `tsara`.                                    |
| **Performance extrême**    | Débit de plus de **1 500 000 mots/seconde** par cœur CPU, sans allocation mémoire superflue.                                                                |

---

## Installation rapide

=== "Python"

    ```bash
    pip install malagasy-stemmer
    ```

=== "Rust (`Cargo.toml`)"

    ```toml
    [dependencies]
    malagasy-stemmer = "0.1"
    ```
