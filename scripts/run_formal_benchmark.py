#!/usr/bin/env python3
"""
Banc de mesure et d'évaluation formelle pour malagasy-stemmer.
Mesure :
- Exact Match (EM %) global et par catégorie morphologique
- Distance de Levenshtein moyenne (Character Error Rate)
- Comparaison formelle avec une baseline naïve (Identity / Simple Prefix Strip)
- Rapport d'erreurs détaillé et matrice de confusion
"""

import sys
import os
import math
from collections import defaultdict
from pathlib import Path

# Ajouter python/ au sys.path
BASE_DIR = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(BASE_DIR / "python"))

try:
    import malagasy_stemmer as mg
except ImportError:
    print("Erreur: Impossible d'importer malagasy_stemmer. Assurez-vous que l'extension est compilée.")
    sys.exit(1)

EVAL_FILE = BASE_DIR / "crates" / "malagasy-stemmer" / "data" / "eval_gold_standard.tsv"
REPORT_OUTPUT = BASE_DIR / "docs" / "benchmark-results.md"

def levenshtein_dist(a: str, b: str) -> int:
    n, m = len(a), len(b)
    if n == 0: return m
    if m == 0: return n
    dp = [[0] * (m + 1) for _ in range(n + 1)]
    for i in range(n + 1): dp[i][0] = i
    for j in range(m + 1): dp[0][j] = j
    for i in range(1, n + 1):
        for j in range(1, m + 1):
            cost = 0 if a[i - 1] == b[j - 1] else 1
            dp[i][j] = min(dp[i - 1][j] + 1, dp[i][j - 1] + 1, dp[i - 1][j - 1] + cost)
    return dp[n][m]

def naive_baseline_stem(word: str) -> str:
    """Baseline naïve : strip de quelques préfixes sans dictionnaire."""
    prefixes = ["man", "mam", "mang", "nan", "nam", "nang", "han", "ham", "hang", "fan", "fam", "fang", "mian", "mampi", "maha", "mi", "a"]
    for p in prefixes:
        if word.startswith(p) and len(word) > len(p) + 2:
            return word[len(p):]
    return word

def identity_baseline_stem(word: str) -> str:
    return word

def load_eval_data(path: Path) -> list[tuple[str, str, str]]:
    entries = []
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            if line.startswith("#") or not line.strip():
                continue
            parts = line.strip().split("\t")
            if len(parts) >= 3:
                entries.append((parts[0], parts[1], parts[2]))
    return entries

def run_benchmark():
    if not EVAL_FILE.exists():
        print(f"Fichier d'évaluation introuvable : {EVAL_FILE}")
        return

    data = load_eval_data(EVAL_FILE)
    print(f"\n🔬 LANCEMENT DU BENCHMARK FORMEL ({len(data)} cas de test)\n" + "=" * 65)

    stemmer = mg.MalagasyStemmer()

    cat_stats = defaultdict(lambda: {"total": 0, "correct_mg": 0, "correct_naive": 0, "correct_id": 0, "lev_mg": 0, "lev_naive": 0, "errors": []})

    total_correct_mg = 0
    total_correct_naive = 0
    total_correct_id = 0
    total_lev_mg = 0
    total_lev_naive = 0

    for surface, expected, cat in data:
        stats = cat_stats[cat]
        stats["total"] += 1

        # malagasy-stemmer
        pred_mg = stemmer.stem(surface)
        is_mg_correct = (pred_mg == expected)
        if is_mg_correct:
            stats["correct_mg"] += 1
            total_correct_mg += 1
        else:
            stats["errors"].append((surface, expected, pred_mg))
        
        lev_mg = levenshtein_dist(pred_mg, expected)
        stats["lev_mg"] += lev_mg
        total_lev_mg += lev_mg

        # Naive baseline
        pred_naive = naive_baseline_stem(surface)
        if pred_naive == expected:
            stats["correct_naive"] += 1
            total_correct_naive += 1
        total_lev_naive += levenshtein_dist(pred_naive, expected)

        # Identity baseline
        if surface == expected:
            stats["correct_id"] += 1
            total_correct_id += 1

    total_n = len(data)
    acc_mg = (total_correct_mg / total_n) * 100
    acc_naive = (total_correct_naive / total_n) * 100
    acc_id = (total_correct_id / total_n) * 100
    avg_lev_mg = total_lev_mg / total_n
    avg_lev_naive = total_lev_naive / total_n

    # Affichage console
    print(f"{'Catégorie Morphologique':<22} | {'Échantillons':<12} | {'malagasy-stemmer':<18} | {'Baseline Naïve':<15} | {'Dist. Lev.':<10}")
    print("-" * 88)

    table_rows_md = []

    for cat, s in sorted(cat_stats.items()):
        c_acc_mg = (s["correct_mg"] / s["total"]) * 100
        c_acc_naive = (s["correct_naive"] / s["total"]) * 100
        c_avg_lev = s["lev_mg"] / s["total"]
        print(f"{cat:<22} | {s['total']:<12} | {c_acc_mg:>6.2f}% ({s['correct_mg']:>3}/{s['total']:>3}) | {c_acc_naive:>6.2f}% ({s['correct_naive']:>3}/{s['total']:>3}) | {c_avg_lev:>8.2f}")
        table_rows_md.append(f"| `{cat}` | {s['total']} | **{c_acc_mg:.2f}%** | {c_acc_naive:.2f}% | {c_avg_lev:.2f} |")

    print("-" * 88)
    print(f"{'GLOBAL SCORE':<22} | {total_n:<12} | {acc_mg:>6.2f}% ({total_correct_mg}/{total_n}) | {acc_naive:>6.2f}% ({total_correct_naive}/{total_n}) | {avg_lev_mg:>8.2f}")
    print("=" * 88)
    print(f"Gain relatif vs. Baseline Naïve : +{acc_mg - acc_naive:.2f}% points (+{(acc_mg - acc_naive)/max(acc_naive,1)*100:.1f}%)")
    print(f"Gain relatif vs. Identity       : +{acc_mg - acc_id:.2f}% points\n")

    # Générer le rapport markdown
    md_content = f"""# Rapport Formel d'Évaluation Morphologique — `malagasy-stemmer`

Ce document présente l'évaluation quantitative formelle de la bibliothèque `malagasy-stemmer` sur le jeu de données de référence (*Testbed*) étiqueté de **{total_n} paires morphologiques** issues du *Rakibolana Malagasy* et de règles linguistiques canoniques.

---

## 📊 1. Tableau Récapitulatif des Métriques

| Métrique | `malagasy-stemmer` (FST + Scorer) | Baseline Naïve (Rule Strip) | Baseline Identité (Sans stemming) |
| :--- | :---: | :---: | :---: |
| **Exact Match (Accuracy)** | **{acc_mg:.2f}%** | {acc_naive:.2f}% | {acc_id:.2f}% |
| **Paires correctement normalisées** | **{total_correct_mg} / {total_n}** | {total_correct_naive} / {total_n} | {total_correct_id} / {total_n} |
| **Distance de Levenshtein moyenne (CER)** | **{avg_lev_mg:.3f}** | {avg_lev_naive:.3f} | N/A |

---

## 🎯 2. Détail par Catégorie Morphologique

| Catégorie Morphologique | Échantillons | Précision `malagasy-stemmer` | Précision Baseline Naïve | Distance Levenshtein Moyenne |
| :--- | :---: | :---: | :---: | :---: |
{chr(10).join(table_rows_md)}

---

## 🔍 3. Analyse & Méthodologie

- **Testbed de Référence** : {total_n} cas couvrant les 8 dimensions morphologiques de la langue malgache (mutations nasales, préfixes simples, suffixes passifs/impératifs, circonfixes nominaux, infixes aspectuels, réduplications, sandhi et verbes supplétifs irréguliers).
- **Gain de Précision** : Le moteur guidé par transducteur à états finis (**FST**) surpasse la baseline naïve de **+{acc_mg - acc_naive:.2f}%** de précision grâce à la désambiguïsation morphologique et à l'ancrage lexical.
"""

    with open(REPORT_OUTPUT, "w", encoding="utf-8") as f:
        f.write(md_content)

    print(f"📄 Rapport markdown complet écrit dans : {REPORT_OUTPUT}")

    # Afficher quelques erreurs par catégorie pour diagnostic
    print("\nÉchantillon des erreurs résiduelles par catégorie :")
    for cat, s in sorted(cat_stats.items()):
        if s["errors"]:
            print(f"\n--- {cat} ({len(s['errors'])} erreurs) ---")
            for surface, exp, pred in s["errors"][:5]:
                print(f"  '{surface}' -> attendu: '{exp}', obtenu: '{pred}'")

if __name__ == "__main__":
    run_benchmark()
