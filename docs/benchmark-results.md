# Rapport Formel d'Évaluation Morphologique — `malagasy-stemmer`

Ce document présente l'évaluation quantitative formelle de la bibliothèque `malagasy-stemmer` sur le jeu de données de référence (*Testbed*) étiqueté de **1393 paires morphologiques** issues du *Rakibolana Malagasy* et de règles linguistiques canoniques.

---

## :fontawesome-solid-chart-simple: 1. Tableau Récapitulatif des Métriques

| Métrique | `malagasy-stemmer` (FST + Scorer) | Baseline Naïve (Rule Strip) | Baseline Identité (Sans stemming) |
| :--- | :---: | :---: | :---: |
| **Exact Match (Accuracy)** | **81.98%** | 17.52% | 0.93% |
| **Paires correctement normalisées** | **1142 / 1393** | 244 / 1393 | 13 / 1393 |
| **Distance de Levenshtein moyenne (CER)** | **0.477** | 2.284 | N/A |

---

## :fontawesome-solid-bullseye: 2. Détail par Catégorie Morphologique

| Catégorie Morphologique | Échantillons | Précision `malagasy-stemmer` | Précision Baseline Naïve | Distance Levenshtein Moyenne |
| :--- | :---: | :---: | :---: | :---: |
| `circumfix` | 300 | **69.67%** | 2.33% | 0.90 |
| `compounds_sandhi` | 17 | **100.00%** | 0.00% | 0.00 |
| `infix` | 110 | **97.27%** | 0.91% | 0.03 |
| `irregular_suppletive` | 39 | **100.00%** | 12.82% | 0.00 |
| `nasal_active` | 300 | **76.67%** | 10.67% | 0.46 |
| `passive_suffix` | 299 | **84.62%** | 1.34% | 0.47 |
| `reduplication` | 21 | **100.00%** | 0.00% | 0.00 |
| `simple_prefix` | 307 | **86.64%** | 63.52% | 0.37 |

---

## :fontawesome-solid-magnifying-glass: 3. Analyse & Méthodologie

- **Testbed de Référence** : 1393 cas couvrant les 8 dimensions morphologiques de la langue malgache (mutations nasales, préfixes simples, suffixes passifs/impératifs, circonfixes nominaux, infixes aspectuels, réduplications, sandhi et verbes supplétifs irréguliers).
- **Gain de Précision** : Le moteur guidé par transducteur à états finis (**FST**) surpasse la baseline naïve de **+64.47%** de précision grâce à la désambiguïsation morphologique et à l'ancrage lexical.
