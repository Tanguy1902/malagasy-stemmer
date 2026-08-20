#!/usr/bin/env python3

import sys
import os
import json
from collections import defaultdict, Counter
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
REPORT_OUTPUT = BASE_DIR / "scripts" / "error_analysis_report.json"


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


def classify_error_pattern(surface: str, expected: str, predicted: str, category: str) -> str:
    """Classifie le pattern d'erreur pour regrouper les cas similaires."""
    
    # Quelle transformation attendait-on ?
    if category == "circumfix":
        # Identifier le type de circonfixe
        for prefix in ["fampif", "fampi", "famp", "faha", "fi", "fa", "fan", "fam", "fang"]:
            if surface.startswith(prefix):
                for suffix in ["ana", "ena", "ina", "ona"]:
                    if surface.endswith(suffix):
                        return f"circumfix:{prefix}..{suffix}"
                return f"circumfix:{prefix}..?"
        return "circumfix:other"
    
    elif category == "nasal_active":
        for prefix in ["mang", "mam", "man", "nang", "nam", "nan", "hang", "ham", "han"]:
            if surface.startswith(prefix):
                return f"nasal:{prefix}"
        return "nasal:other"
    
    elif category == "passive_suffix":
        for suffix in ["ana", "ina", "ena", "ona"]:
            if surface.endswith(suffix):
                return f"suffix:{suffix}"
        return "suffix:other"
    
    elif category == "simple_prefix":
        for prefix in ["mi", "mpi", "fi", "faha", "maha", "tafa", "a", "i"]:
            if surface.startswith(prefix):
                return f"simple_prefix:{prefix}"
        return "simple_prefix:other"
    
    elif category == "compounds_sandhi":
        return "compound"
    
    elif category == "infix":
        return "infix"
    
    return category


def analyze_morphophonemic_error(surface: str, expected: str, predicted: str) -> str:
    """Analyse le type de restauration morphophonémique qui a échoué."""
    
    # Comparer les fins de expected vs predicted
    if expected.endswith("tra") and not predicted.endswith("tra"):
        return "restoration_tra_missing"
    if expected.endswith("ka") and not predicted.endswith("ka"):
        return "restoration_ka_missing"
    if expected.endswith("y") and not predicted.endswith("y"):
        return "restoration_y_missing"
    if expected.endswith("na") and not predicted.endswith("na"):
        return "restoration_na_missing"
    
    # Le predicted est le surface lui-même (identity fallback)
    if predicted == surface.lower():
        return "identity_fallback"
    
    # Le predicted est trop long (sur-stemming ou sous-stemming)
    if len(predicted) > len(expected) + 2:
        return "under_stemmed"
    if len(predicted) < len(expected) - 2:
        return "over_stemmed"
    
    # Erreur de mutation nasale
    if predicted and expected:
        if predicted[0] != expected[0]:
            return f"wrong_initial_consonant:{predicted[0]}_vs_{expected[0]}"
    
    return "other"


def run_deep_analysis():
    if not EVAL_FILE.exists():
        print(f"Fichier d'évaluation introuvable : {EVAL_FILE}")
        return

    data = load_eval_data(EVAL_FILE)
    stemmer = mg.MalagasyStemmer()

    # === STATISTIQUES GLOBALES ===
    total = len(data)
    correct = 0
    errors_by_category = defaultdict(list)
    category_totals = defaultdict(int)
    
    # === CLASSIFICATION DES ERREURS ===
    generation_errors = []  # Le bon candidat n'a jamais été produit
    scoring_errors = []      # Le bon candidat existait mais pas sélectionné
    
    error_pattern_counter = Counter()
    morpho_error_counter = Counter()
    
    # Erreurs par distance de Levenshtein
    lev_distribution = Counter()
    
    for surface, expected, cat in data:
        category_totals[cat] += 1
        result = stemmer.stem_with_details(surface)
        predicted = result.root
        
        if predicted == expected:
            correct += 1
            continue
        
        lev = levenshtein_dist(predicted, expected)
        lev_distribution[lev] += 1
        
        error_info = {
            "surface": surface,
            "expected": expected,
            "predicted": predicted,
            "category": cat,
            "operation": result.operation,
            "confidence": result.confidence,
            "in_dictionary": result.in_dictionary,
            "levenshtein": lev,
        }
        
        errors_by_category[cat].append(error_info)
        
        # Classifier le pattern
        pattern = classify_error_pattern(surface, expected, predicted, cat)
        error_pattern_counter[pattern] += 1
        
        # Classifier l'erreur morphophonémique
        morpho = analyze_morphophonemic_error(surface, expected, predicted)
        morpho_error_counter[morpho] += 1
        error_info["morpho_error_type"] = morpho
        error_info["error_pattern"] = pattern
        
        # Déterminer si c'est une erreur de génération ou de scoring
        # On ne peut pas facilement accéder aux candidats internes depuis Python,
        # mais on peut inférer :
        # - Si predicted == surface (identity fallback) → probablement erreur de génération
        # - Si predicted est dans le dictionnaire mais wrong → erreur de scoring
        # - Si predicted n'est pas dans le dictionnaire → erreur de génération (aucun bon candidat en dict)
        
        if predicted == surface.lower():
            error_info["error_class"] = "generation_failure_identity"
            generation_errors.append(error_info)
        elif not result.in_dictionary and result.operation == "identity_fallback":
            error_info["error_class"] = "generation_failure_no_candidate"
            generation_errors.append(error_info)
        elif result.in_dictionary and predicted != expected:
            # Le stemmer a trouvé un mot du dictionnaire, mais pas le bon
            error_info["error_class"] = "scoring_wrong_dict_word"
            scoring_errors.append(error_info)
        else:
            # Le stemmer a produit un candidat hors dictionnaire
            error_info["error_class"] = "generation_partial"
            generation_errors.append(error_info)

    total_errors = total - correct
    accuracy = (correct / total) * 100

    # === RAPPORT CONSOLE ===
    print(f"\n{'='*80}")
    print(f"  ANALYSE D'ERREURS SYSTÉMATIQUE — malagasy-stemmer")
    print(f"  {total} cas de test | {correct} corrects | {total_errors} erreurs")
    print(f"  Score global : {accuracy:.2f}%")
    print(f"{'='*80}\n")

    # 1. Score par catégorie
    print("═══ 1. PRÉCISION PAR CATÉGORIE ═══\n")
    print(f"{'Catégorie':<22} | {'Total':<6} | {'Correct':<8} | {'Erreurs':<8} | {'Précision':<10}")
    print("-" * 65)
    for cat in sorted(category_totals.keys()):
        n = category_totals[cat]
        errs = len(errors_by_category[cat])
        corr = n - errs
        acc = (corr / n) * 100
        bar = "█" * int(acc / 5) + "░" * (20 - int(acc / 5))
        print(f"{cat:<22} | {n:<6} | {corr:<8} | {errs:<8} | {acc:>6.2f}% {bar}")
    
    # 2. Classification des erreurs
    print(f"\n═══ 2. CLASSIFICATION DES ERREURS ═══\n")
    print(f"  Erreurs de GÉNÉRATION (candidat correct jamais produit) : {len(generation_errors)}")
    print(f"  Erreurs de SCORING (candidat correct produit mais pas choisi) : {len(scoring_errors)}")
    
    gen_pct = len(generation_errors) / max(total_errors, 1) * 100
    score_pct = len(scoring_errors) / max(total_errors, 1) * 100
    print(f"  → {gen_pct:.1f}% génération | {score_pct:.1f}% scoring\n")
    
    # Sous-classes
    gen_classes = Counter(e["error_class"] for e in generation_errors)
    score_classes = Counter(e["error_class"] for e in scoring_errors)
    
    print("  Détail des erreurs de génération :")
    for cls, count in gen_classes.most_common():
        print(f"    {cls}: {count}")
    
    print("\n  Détail des erreurs de scoring :")
    for cls, count in score_classes.most_common():
        print(f"    {cls}: {count}")

    # 3. Top patterns d'erreurs
    print(f"\n═══ 3. TOP 15 PATTERNS D'ERREURS ═══\n")
    for pattern, count in error_pattern_counter.most_common(15):
        print(f"  [{count:>3}] {pattern}")
    
    # 4. Types d'erreurs morphophonémiques
    print(f"\n═══ 4. TYPES D'ERREURS MORPHOPHONÉMIQUES ═══\n")
    for mtype, count in morpho_error_counter.most_common(15):
        print(f"  [{count:>3}] {mtype}")
    
    # 5. Distribution des distances de Levenshtein
    print(f"\n═══ 5. DISTRIBUTION DES DISTANCES DE LEVENSHTEIN ═══\n")
    for dist in sorted(lev_distribution.keys()):
        count = lev_distribution[dist]
        bar = "█" * min(count, 60)
        print(f"  dist={dist}: {count:>4} {bar}")
    
    # 6. Échantillons détaillés par catégorie (les plus instructifs)
    print(f"\n═══ 6. ÉCHANTILLONS D'ERREURS DÉTAILLÉS PAR CATÉGORIE ═══\n")
    for cat in sorted(errors_by_category.keys()):
        errs = errors_by_category[cat]
        n_cat = category_totals[cat]
        acc = ((n_cat - len(errs)) / n_cat) * 100
        print(f"\n--- {cat} ({len(errs)} erreurs / {n_cat} total, précision {acc:.1f}%) ---")
        
        # Trier par fréquence du pattern d'erreur
        for e in errs[:15]:
            dict_marker = "📗" if e["in_dictionary"] else "📕"
            print(f"  {dict_marker} '{e['surface']}' → attendu: '{e['expected']}' | obtenu: '{e['predicted']}' "
                  f"[op={e['operation']}, conf={e['confidence']:.2f}, lev={e['levenshtein']}, "
                  f"type={e.get('morpho_error_type', '?')}]")
    
    # 7. Analyse spéciale : mots où le dictionnaire manque
    print(f"\n═══ 7. RACINES ATTENDUES MANQUANTES DANS LE DICTIONNAIRE ═══\n")
    missing_roots = Counter()
    for cat_errs in errors_by_category.values():
        for e in cat_errs:
            # Vérifier si la racine attendue est dans le dictionnaire
            try:
                lookup = stemmer.stem_with_details(e["expected"])
                if lookup.root == e["expected"] and lookup.in_dictionary:
                    pass  # OK, la racine est dans le dict
                else:
                    missing_roots[e["expected"]] += 1
            except:
                missing_roots[e["expected"]] += 1
    
    if missing_roots:
        print(f"  {len(missing_roots)} racines attendues possiblement absentes du dictionnaire :")
        for root, count in missing_roots.most_common(30):
            print(f"    [{count:>2}x] {root}")
    else:
        print("  Toutes les racines attendues semblent présentes dans le dictionnaire.")
    
    # 8. Opérations du stemmer sur les erreurs
    print(f"\n═══ 8. OPÉRATIONS CHOISIES SUR LES ERREURS ═══\n")
    op_counter = Counter(e["operation"] for errs in errors_by_category.values() for e in errs)
    for op, count in op_counter.most_common():
        print(f"  [{count:>3}] {op}")

    # === SAUVEGARDER LE RAPPORT JSON ===
    report = {
        "summary": {
            "total_cases": total,
            "correct": correct,
            "errors": total_errors,
            "accuracy_pct": round(accuracy, 2),
            "generation_errors": len(generation_errors),
            "scoring_errors": len(scoring_errors),
        },
        "accuracy_by_category": {
            cat: {
                "total": category_totals[cat],
                "errors": len(errors_by_category[cat]),
                "accuracy_pct": round(((category_totals[cat] - len(errors_by_category[cat])) / category_totals[cat]) * 100, 2)
            }
            for cat in sorted(category_totals.keys())
        },
        "top_error_patterns": error_pattern_counter.most_common(20),
        "morpho_error_types": morpho_error_counter.most_common(20),
        "levenshtein_distribution": dict(sorted(lev_distribution.items())),
        "all_errors": [e for errs in errors_by_category.values() for e in errs],
    }

    with open(REPORT_OUTPUT, "w", encoding="utf-8") as f:
        json.dump(report, f, ensure_ascii=False, indent=2)
    
    print(f"\n[OK] Rapport JSON détaillé sauvegardé dans : {REPORT_OUTPUT}")


if __name__ == "__main__":
    run_deep_analysis()
