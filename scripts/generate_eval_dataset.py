#!/usr/bin/env python3
"""
Générateur du Dataset d'Évaluation de Référence (Testbed) pour malagasy-stemmer.
Extrait des paires morphologiques étiquetées (forme de surface -> racine attendue)
stratifiées par catégorie linguistique :
- nasal_active (man-, mam-, mang-, nan-, han-, fan-, mpan-)
- simple_prefix (mi-, fi-, maha-, tafa-, a-, etc.)
- passive_suffix (-ina, -ana, -ena, -y, -o)
- circumfix (fan-...-ana, fi-...-ana, fampi-...-ana, faha-...-ana)
- infix (-in-, -om-)
- reduplication (exacte & alternante)
- compounds_sandhi (mots composés avec sandhi)
- irregular_suppletive (verbes et formes supplétives)
"""

import os
import re
import sqlite3
import random
from collections import defaultdict
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent.parent
DB_PATH = Path("/home/fanilo/.gemini/antigravity-ide/brain/f1a23a37-7f6b-4e3a-9616-b586426c9191/scratch/rakibolana_malagasy/sqlite/rakibolana.db")
ROOTS_TSV = BASE_DIR / "crates" / "malagasy-stemmer" / "data" / "roots.tsv"
EVAL_DATASET_PATH = BASE_DIR / "crates" / "malagasy-stemmer" / "data" / "eval_gold_standard.tsv"

MALAGASY_INVALID_LETTERS = set("cqwx")
DIALECT_MARKERS = {
    "btl", "sak", "ant", "tsim", "bara", "bet", "antais", "tanala", "vezo",
    "sakalava", "betsimisaraka", "merina", "mah", "j.f", "jf", "antandroy", "ski"
}

# 1. Paires vérifiées manuellement (Golden Standard ancré)
CURATED_GOLDEN_PAIRS = [
    # Mutations nasales actives
    ("manoratra", "soratra", "nasal_active"),
    ("nanoratra", "soratra", "nasal_active"),
    ("hanoratra", "soratra", "nasal_active"),
    ("fanoratana", "soratra", "nasal_active"),
    ("mpanoratra", "soratra", "nasal_active"),
    ("mamaky", "vaky", "nasal_active"),
    ("namaky", "vaky", "nasal_active"),
    ("hamaky", "vaky", "nasal_active"),
    ("mpamaky", "vaky", "nasal_active"),
    ("mamboly", "voly", "nasal_active"),
    ("namboly", "voly", "nasal_active"),
    ("fambolena", "voly", "nasal_active"),
    ("mpamboly", "voly", "nasal_active"),
    ("mangalatra", "halatra", "nasal_active"),
    ("nangalatra", "halatra", "nasal_active"),
    ("fangalatra", "halatra", "nasal_active"),
    ("mandatsaka", "latsaka", "nasal_active"),
    ("mandroso", "roso", "nasal_active"),
    ("fandrosoana", "roso", "circumfix"),
    ("manjaka", "zaka", "nasal_active"),
    ("fanjakana", "zaka", "circumfix"),
    ("manenjana", "henjana", "nasal_active"),
    ("manaja", "haja", "nasal_active"),
    ("mamotsy", "fotsy", "nasal_active"),
    ("mamaraingo", "faraingo", "nasal_active"),

    # Préfixes simples
    ("mianatra", "anatra", "simple_prefix"),
    ("mpianatra", "anatra", "simple_prefix"),
    ("mahafantatra", "fantatra", "simple_prefix"),
    ("mahasalama", "salama", "simple_prefix"),
    ("tafapetraka", "petraka", "simple_prefix"),
    ("ababy", "baby", "simple_prefix"),
    ("abahana", "bahana", "simple_prefix"),
    ("aaloka", "aloka", "simple_prefix"),
    ("aampatra", "ampatra", "simple_prefix"),
    ("aankina", "ankina", "simple_prefix"),
    ("fiseho", "seho", "simple_prefix"),

    # Circonfixes
    ("fampianarana", "anatra", "circumfix"),
    ("fahasalamana", "salama", "circumfix"),
    ("fahendrena", "hendry", "circumfix"),
    ("fihotakotahana", "hotakotaka", "circumfix"),
    ("fisehoana", "seho", "circumfix"),
    ("fidirana", "iditra", "circumfix"),
    ("fidinana", "dina", "circumfix"),

    # Passifs et suffixes
    ("soratana", "soratra", "passive_suffix"),
    ("vakina", "vaky", "passive_suffix"),
    ("anarana", "anatra", "passive_suffix"),
    ("tapahana", "tapaka", "passive_suffix"),
    ("tenenina", "teny", "passive_suffix"),
    ("tsindriana", "tsindry", "passive_suffix"),
    ("lazaina", "laza", "passive_suffix"),
    ("fidina", "fidy", "passive_suffix"),
    ("babena", "baby", "passive_suffix"),
    ("soraty", "soratra", "passive_suffix"),
    ("vakio", "vaky", "passive_suffix"),
    ("tapaho", "tapaka", "passive_suffix"),
    ("fidio", "fidy", "passive_suffix"),

    # Infixes
    ("vinaky", "vaky", "infix"),
    ("tinapaka", "tapaka", "infix"),
    ("jinery", "jery", "infix"),

    # Réduplications
    ("tsaratsara", "tsara", "reduplication"),
    ("moramora", "mora", "reduplication"),
    ("kelykely", "kely", "reduplication"),
    ("fotsifotsy", "fotsy", "reduplication"),
    ("madinidinika", "madinika", "reduplication"),
    ("maintimainty", "mainty", "reduplication"),
    ("menamena", "mena", "reduplication"),

    # Mots composés et Sandhi
    ("harem-pirenena", "harena_firenena", "compounds_sandhi"),
    ("tanan-dehibe", "tanana_lehibe", "compounds_sandhi"),
    ("ara-potoana", "araka_fotoana", "compounds_sandhi"),
    ("ara-toekarena", "araka_toekarena", "compounds_sandhi"),
    ("ara-politika", "araka_politika", "compounds_sandhi"),
    ("ara-governemanta", "araka_governemanta", "compounds_sandhi"),
    ("ara-teknolojia", "araka_teknolojia", "compounds_sandhi"),
    ("ara-demokrasia", "araka_demokrasia", "compounds_sandhi"),
    ("tany-fambolena", "tany_voly", "compounds_sandhi"),
    ("mpiara-monina", "miara_monina", "compounds_sandhi"),

    # Irréguliers et supplétifs
    ("mandeha", "leha", "irregular_suppletive"),
    ("nandeha", "leha", "irregular_suppletive"),
    ("handeha", "leha", "irregular_suppletive"),
    ("fandehanana", "leha", "irregular_suppletive"),
    ("mandehana", "leha", "irregular_suppletive"),
    ("homana", "hano", "irregular_suppletive"),
    ("mihinana", "hano", "irregular_suppletive"),
    ("hanina", "hano", "irregular_suppletive"),
    ("nohanina", "hano", "irregular_suppletive"),
    ("fihinanana", "hano", "irregular_suppletive"),
    ("entina", "tondra", "irregular_suppletive"),
    ("nentina", "tondra", "irregular_suppletive"),
    ("ento", "tondra", "irregular_suppletive"),
    ("maka", "aka", "irregular_suppletive"),
    ("alaina", "aka", "irregular_suppletive"),
    ("nalaina", "aka", "irregular_suppletive"),
    ("alao", "aka", "irregular_suppletive"),
    ("manome", "ome", "irregular_suppletive"),
    ("omena", "ome", "irregular_suppletive"),
    ("nomena", "ome", "irregular_suppletive"),
    ("omeo", "ome", "irregular_suppletive"),
    ("amidy", "varotra", "irregular_suppletive"),
    ("namidy", "varotra", "irregular_suppletive"),
    ("hamidy", "varotra", "irregular_suppletive"),
    ("famidiana", "varotra", "irregular_suppletive"),
    ("misy", "misy", "irregular_suppletive"),
    ("nisy", "misy", "irregular_suppletive"),
    ("fisiana", "misy", "irregular_suppletive"),
    ("mandre", "re", "irregular_suppletive"),
    ("nandre", "re", "irregular_suppletive"),
    ("renesina", "re", "irregular_suppletive"),
    ("mamoaka", "voaka", "irregular_suppletive"),
    ("avoaka", "voaka", "irregular_suppletive"),
    ("miditra", "iditra", "irregular_suppletive"),
    ("ampidiro", "iditra", "irregular_suppletive"),
    ("miakatra", "akatra", "irregular_suppletive"),
    ("akarina", "akatra", "irregular_suppletive"),
    ("midina", "dina", "irregular_suppletive"),
    ("ampidinina", "dina", "irregular_suppletive"),
]

def clean_word(w: str) -> str:
    w = w.lower().strip()
    w = re.sub(r'[^a-z\-]', '', w)
    return w

def is_valid_malagasy_word(w: str) -> bool:
    if not w or len(w) < 2 or len(w) > 18:
        return False
    if any(c in MALAGASY_INVALID_LETTERS for c in w if c != '-'):
        return False
    if not re.search(r'([aeioy]|ka|tra|na|ra)$', w):
        return False
    return True

def categorize_pair(surface: str, root: str) -> str:
    if '-' in surface:
        return "compounds_sandhi"
    if surface.startswith(root) and len(surface) > len(root):
        rem = surface[len(root):]
        if rem == root or rem == root[:-1] + 'y' or root.startswith(rem):
            return "reduplication"
    if re.match(r'^(fan|fam|fang|famp|fampi|fi|faha|ha)[a-z]+(ana|ina|ena|ona)$', surface):
        return "circumfix"
    if re.match(r'^(man|mam|mang|nan|nam|nang|han|ham|hang|fan|fam|fang|mpan|mpam|mpang)', surface):
        return "nasal_active"
    if re.match(r'^(mi|maha|tafa|mif|mpi|a|ma|fa|ha|i)', surface):
        return "simple_prefix"
    if surface.endswith('ana') or surface.endswith('ina') or surface.endswith('ena') or surface.endswith('ona') or surface.endswith('y') or surface.endswith('o'):
        return "passive_suffix"
    if 'in' in surface[1:4] or 'om' in surface[1:4]:
        return "infix"
    return "simple_prefix"

def build_evaluation_dataset(target_size_per_cat=300):
    print("Extraction du dataset d'évaluation de référence...")
    pairs_by_cat = defaultdict(set)

    # 1. Charger les racines autorisées depuis roots.tsv
    valid_roots = set()
    if ROOTS_TSV.exists():
        with open(ROOTS_TSV) as f:
            for line in f:
                if not line.startswith("#") and line.strip():
                    valid_roots.add(line.split("\t")[0].strip().lower())

    # 2. Ajouter d'abord les paires dorées ancrées
    for s, r, c in CURATED_GOLDEN_PAIRS:
        pairs_by_cat[c].add((s, r))

    # 3. Extraire depuis Rakibolana SQLite
    if DB_PATH.exists():
        conn = sqlite3.connect(DB_PATH)
        c = conn.cursor()
        rows = c.execute("SELECT word, definition FROM rakibolana").fetchall()
        conn.close()

        random.seed(42)
        shuffled_rows = list(rows)
        random.shuffle(shuffled_rows)

        for raw_w, defn in shuffled_rows:
            w = clean_word(raw_w)
            if not is_valid_malagasy_word(w):
                continue
            
            m = re.search(r'^\s*\(([^)]+)\)', defn)
            if m:
                raw_inside = m.group(1).strip()
                # Filtrer les notes étymologiques comme "fr. dynamite" ou "ski"
                if raw_inside.lower().startswith("fr") or raw_inside.lower().startswith("eng") or raw_inside.lower().startswith("ar"):
                    continue
                inside = clean_word(raw_inside)
                if inside in DIALECT_MARKERS or len(inside) < 2 or not is_valid_malagasy_word(inside):
                    continue
                if inside not in valid_roots:
                    continue
                if w == inside:
                    continue
                
                cat = categorize_pair(w, inside)
                if len(pairs_by_cat[cat]) < target_size_per_cat:
                    pairs_by_cat[cat].add((w, inside))

    # Rassembler et trier
    all_eval_entries = []
    for cat, pair_set in sorted(pairs_by_cat.items()):
        print(f"  - {cat:20} : {len(pair_set)} paires")
        for s, r in sorted(pair_set):
            all_eval_entries.append((s, r, cat))

    print(f"\nTotal des cas de test d'évaluation : {len(all_eval_entries)}")

    # Écriture du fichier TSV
    header = "# Dataset d'Évaluation Formelle pour malagasy-stemmer\n# Format: surface_form\\texpected_root\\tcategory\n"
    with open(EVAL_DATASET_PATH, "w", encoding="utf-8") as f:
        f.write(header)
        for s, r, cat in all_eval_entries:
            f.write(f"{s}\t{r}\t{cat}\n")

    print(f"[OK] Fichier d'évaluation généré : {EVAL_DATASET_PATH}")
    print(f"Total des paires morphologiques : {len(all_eval_entries)}")

if __name__ == "__main__":
    build_evaluation_dataset()
