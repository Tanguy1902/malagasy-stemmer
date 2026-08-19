# Rapport Formel d'Évaluation Morphologique — `malagasy-stemmer`

Ce document présente l'évaluation quantitative formelle de la bibliothèque `malagasy-stemmer` sur le jeu de données de référence (*Testbed*) étiqueté de **1387 paires morphologiques** issues du *Rakibolana Malagasy* et de règles linguistiques canoniques.

---

## 📊 1. Tableau Récapitulatif des Métriques

| Métrique | `malagasy-stemmer` (FST + Scorer) | Baseline Naïve (Rule Strip) | Baseline Identité (Sans stemming) |
| :--- | :---: | :---: | :---: |
| **Exact Match (Accuracy)** | **74.91%** | 16.87% | 0.07% |
| **Paires correctement normalisées** | **1039 / 1387** | 234 / 1387 | 1 / 1387 |
| **Distance de Levenshtein moyenne (CER)** | **0.717** | 2.339 | N/A |

---

## 🎯 2. Détail par Catégorie Morphologique

| Catégorie Morphologique | Échantillons | Précision `malagasy-stemmer` | Précision Baseline Naïve | Distance Levenshtein Moyenne |
| :--- | :---: | :---: | :---: | :---: |
| `circumfix` | 300 | **64.67%** | 2.33% | 1.21 |
| `compounds_sandhi` | 17 | **47.06%** | 0.00% | 1.76 |
| `infix` | 110 | **91.82%** | 0.00% | 0.11 |
| `irregular_suppletive` | 39 | **100.00%** | 12.82% | 0.00 |
| `nasal_active` | 300 | **72.67%** | 10.67% | 0.53 |
| `passive_suffix` | 300 | **72.33%** | 0.00% | 0.85 |
| `reduplication` | 21 | **100.00%** | 0.00% | 0.00 |
| `simple_prefix` | 300 | **80.33%** | 63.33% | 0.58 |

---

## 🔍 3. Analyse & Méthodologie

- **Testbed de Référence** : 1387 cas couvrant les 8 dimensions morphologiques de la langue malgache (mutations nasales, préfixes simples, suffixes passifs/impératifs, circonfixes nominaux, infixes aspectuels, réduplications, sandhi et verbes supplétifs irréguliers).
- **Gain de Précision** : Le moteur guidé par transducteur à états finis (**FST**) surpasse la baseline naïve de **+58.04%** de précision grâce à la désambiguïsation morphologique et à l'ancrage lexical.
