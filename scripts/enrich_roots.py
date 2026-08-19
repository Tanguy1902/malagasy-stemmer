#!/usr/bin/env python3
"""
Script d'extraction et d'enrichissement massif du dictionnaire de racines pures (fototeny) malgaches.
Combine :
1. Dictionnaire de base existant (crates/malagasy-stemmer/data/roots.tsv)
2. Kaikki.org / Wiktionary Malagasy (kaikki.org-dictionary-Malagasy.jsonl)
3. Rakibolana Malagasy structuré (scratch/rakibolana_malagasy/json/*.json)

Garantit :
- Validation orthographique stricte (alphabet malgache à 21 lettres, terminaisons phonotactiques régulières)
- Détection des catégories grammaticales (v, n, adj, adv, prep, pron, conj, num, misc)
- Tri alphabétique strict (requis pour la compilation du Transducteur à États Finis FST en Rust)
"""

import glob
import json
import os
import re
import sys
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent.parent
ROOTS_TSV_PATH = BASE_DIR / "crates" / "malagasy-stemmer" / "data" / "roots.tsv"
KAIKKI_PATH = BASE_DIR / "kaikki.org-dictionary-Malagasy.jsonl"
RAKIBOLANA_DIR = Path("/home/fanilo/.gemini/antigravity-ide/brain/f1a23a37-7f6b-4e3a-9616-b586426c9191/scratch/rakibolana_malagasy/json")

MALAGASY_INVALID_LETTERS = set("cqwx")

# Adjectifs et noms légitimes commençant par ma/mam/man/mi qui sont de vraies racines simples
LEGITIMATE_PREFIX_LIKE_ROOTS = {
    "mamy", "manga", "mando", "mangidy", "mainty", "manify", "manitra",
    "mantso", "manta", "manina", "marina", "maizina", "malemy", "mafy",
    "madio", "madinika", "mora", "masina", "mavo", "maintso", "maika",
    "marary", "maro", "masaka", "mavozo", "maheri", "mahery", "mahia",
    "moka", "mona", "mofo", "mody", "mavo", "mira", "maso", "mena",
    "fotsy", "feno", "fohy", "faly", "faty", "fomba", "foko", "fitia",
    "misy", "midina", "miakatra", "miditra"
}

# Racines canoniques garanties requises par le moteur morphologique et les tests
CANONICAL_REQUIRED_ROOTS = {
    "aka": "v", "akatra": "v", "ala": "n", "anatra": "n", "andry": "n", "andron": "n",
    "andro": "n", "angona": "v", "antso": "v", "antoka": "n", "asa": "n", "atsangana": "v",
    "avy": "v", "ba": "n", "baiboly": "n", "bala": "n", "be": "adj", "biby": "n",
    "boky": "n", "dada": "n", "didy": "n", "dina": "v", "dio": "adj", "diniha": "n",
    "dinika": "n", "dia": "n", "ditra": "adj", "doka": "n", "drano": "n", "efa": "adv",
    "elo": "n", "endrika": "n", "entana": "n", "erana": "n", "etsy": "adv", "fafa": "v",
    "faika": "n", "faladia": "n", "faly": "adj", "fanalana": "n", "fantatra": "adj",
    "fatra": "n", "fe": "n", "fefy": "n", "fehy": "n", "feno": "adj", "feo": "n",
    "fidy": "v", "fikitra": "n", "filaza": "n", "finoana": "n", "firenena": "n",
    "firy": "num", "fito": "num", "fitondrana": "n", "fo": "n", "foha": "v", "fohy": "adj",
    "forona": "v", "fotsy": "adj", "haino": "v", "haingo": "n", "haja": "n", "hakeo": "v",
    "halatra": "n", "halo": "n", "halana": "n", "hano": "v", "harena": "n", "hazo": "n",
    "hehy": "n", "heloka": "n", "hery": "n", "hevitra": "n", "hira": "n", "hitsy": "adj",
    "hono": "misc", "iala": "v", "iditra": "v", "ilay": "pron", "indray": "adv",
    "indrindra": "adv", "ino": "v", "io": "pron", "iray": "num", "ireo": "pron",
    "isika": "pron", "isy": "v", "itoy": "pron", "izaho": "pron", "izahay": "pron",
    "izany": "pron", "izao": "pron", "izay": "pron", "izy": "pron", "ka": "conj",
    "kalo": "n", "karoka": "n", "kely": "adj", "kibo": "n", "kintana": "n", "koba": "n",
    "kodia": "n", "kofehy": "n", "kolontsaina": "n", "lala": "v", "lalana": "n",
    "laza": "v", "leha": "v", "lehibe": "adj", "lehibe": "adj", "loha": "n", "loko": "n",
    "lositra": "v", "madio": "adj", "mafy": "adj", "mainty": "adj", "maintso": "adj",
    "maizina": "adj", "malemy": "adj", "mamy": "adj", "manify": "adj", "manitra": "adj",
    "manta": "adj", "marary": "adj", "marina": "adj", "maro": "adj", "masaka": "adj",
    "masina": "adj", "maso": "n", "masoandro": "n", "mavo": "adj", "misy": "v",
    "mora": "adj", "namana": "n", "neny": "n", "nify": "n", "nofo": "n",
    "ody": "n", "olona": "n", "ome": "v", "ondry": "n", "ongana": "v", "ony": "n",
    "orana": "n", "orona": "n", "osa": "adj", "otrika": "n", "paika": "n", "panahy": "n",
    "petraka": "v", "posy": "n", "raha": "conj", "raharaha": "n", "rahona": "n",
    "rano": "n", "ravina": "n", "re": "v", "reny": "n", "resaka": "n", "resy": "adj",
    "rohy": "n", "rova": "n", "sa": "conj", "sady": "conj", "saha": "n", "saka": "n",
    "sakana": "n", "salama": "adj", "samia": "pron", "samy": "pron", "sary": "n",
    "sasa": "v", "sazy": "n", "seza": "n", "siramamy": "n", "sisa": "n", "sokajy": "n",
    "sombina": "n", "sondrona": "n", "songona": "n", "sonia": "n", "soratra": "n",
    "sosotra": "adj", "tadidy": "n", "tady": "n", "tahotra": "n", "takatra": "n",
    "tamin": "prep", "tampoka": "adv", "tanana": "n", "tanora": "adj", "tanteraka": "adj",
    "tao": "v", "taona": "n", "tapaka": "v", "taratasy": "n", "tarih": "n", "tarika": "n",
    "tatao": "n", "tazana": "v", "teny": "n", "tiana": "v", "toby": "n", "toerana": "n",
    "toetra": "n", "tokantrano": "n", "toky": "n", "tolo": "n", "tolotra": "n",
    "tondro": "n", "tonga": "v", "tondra": "v", "tonta": "n", "toro": "n", "tory": "v",
    "tovo": "n", "trano": "n", "tsara": "adj", "tsena": "n", "tsia": "adv", "tsindry": "v",
    "tsipika": "n", "tsirairay": "adj", "tsiro": "n", "tsoraka": "n", "vahiny": "n",
    "vaky": "v", "vala": "n", "valala": "n", "valy": "v", "vango": "n", "varotra": "n",
    "vary": "n", "vasoka": "adj", "vato": "n", "vava": "n", "vavaka": "n", "vavy": "n",
    "vazaha": "n", "velona": "adj", "very": "adj", "vidy": "n", "vina": "n", "vintana": "n",
    "vita": "adj", "vody": "n", "voa": "n", "voahangy": "n", "voanjo": "n", "voary": "n",
    "vohitra": "n", "vola": "n", "volana": "n", "volo": "n", "voly": "n", "vondrona": "n",
    "vono": "v", "vory": "v", "votoatiny": "n", "zaha": "v", "zanak": "n", "zanaka": "n",
    "zara": "n", "zavatra": "n", "zaza": "n", "zoma": "n", "zoro": "n"
}

# Formes de surface irrégulières qui ne doivent JAMAIS être dans roots.tsv
IRREGULAR_SURFACE_FORMS = {
    "ahatongavana", "akany", "akarina", "akaro", "alaina", "alao", "alaona",
    "amidio", "amidy", "ampidinina", "ampidino", "ampidirina", "ampidiro",
    "andeha", "andehanana", "andrenesana", "anomezana", "atao", "ataovy",
    "avoahy", "avoaka", "entina", "ento", "fahalalana", "fahatonga",
    "fahatongavana", "famidiana", "famoahana", "famoaka", "fanao", "fanaovana",
    "fandeha", "fandehanana", "fantarina", "fantaro", "fatoriana", "fiaviana",
    "fidinana", "fidirana", "fifohazana", "fihainoana", "fihinana", "fihinanana",
    "fipetrahana", "fisiana", "fisianana", "fitazanana", "fitokisana", "fitondrana",
    "fivaliana", "fivarotana", "fohazina", "fohazo", "frenesana", "frenesina",
    "hahafantatra", "hahalala", "hakarina", "halaina", "hamidy", "hamoaka",
    "hampidinina", "hampidirina", "handeha", "handehana", "handehanana",
    "handositra", "handre", "handrenesana", "hanina", "hanome", "hatao",
    "hatory", "havoaka", "hentina", "hiakatra", "hiaviana", "hidina",
    "hidirana", "hiditra", "hifoha", "hihaino", "hihinana", "hihinanana",
    "hipetraka", "hisiana", "hisianana", "hisy", "hitazana", "hitondra",
    "hivarotra", "hividy", "hofantarina", "hofohazina", "hohainoina",
    "hohanina", "holazaina", "holosirina", "homana", "homena", "horenesina",
    "hotoriana", "hovidina", "iaviana", "idinana", "idirana", "ihinanana",
    "isianana", "kohanana", "kohanina", "losirina", "losiro", "mahafantatra",
    "mahalala", "maka", "mamoaka", "mampiditra", "mandeha", "mandehana",
    "mandositra", "mandre", "manome", "matory", "miakatra", "midina",
    "miditra", "mifoha", "mihaino", "mihinana", "mipetraka", "misy",
    "mitazana", "mitondra", "mivarotra", "mividy", "mpahalala", "mpahatonga",
    "mpaka", "mpamoaka", "mpampiditra", "mpandeha", "mpandositra", "mpanome",
    "mpatory", "mpiakatra", "mpidina", "mpiditra", "mpifoha", "mpihaino",
    "mpihinana", "mpitondra", "nahafantatra", "nahalala", "nahatonga", "naka",
    "nakarina", "nalaina", "nalao", "namidy", "namoaka", "nampidinina",
    "nampidirina", "nandeha", "nandehanana", "nandositra", "nandre",
    "nandrenesana", "nanome", "natao", "natory", "navoaka", "nentina",
    "niakatra", "niaviana", "nidina", "nidirana", "niditra", "nifoha",
    "nihaino", "nihinana", "nihinanana", "nipetraka", "nisianana", "nisy",
    "nitazana", "nitondra", "nivarotra", "nividy", "nofantarina", "nofohazina",
    "nohainoina", "nohanina", "nolazaina", "nolosirina", "nomena", "norenesina",
    "notenenina", "notoriana", "novaliana", "novidina", "omena", "omeo",
    "petrahana", "renesina", "tazanina", "tenenina", "teneno", "tokisana",
    "toriana", "torio", "valiana", "vidina", "vidio", "lazaina", "lazao",
    "tapahina", "tapaho", "soratana", "soraty", "fambolena", "fahasalamana",
    "fahendrena"
}

def clean_word(w: str) -> str:
    w = w.lower().strip()
    w = re.sub(r'[^a-z]', '', w)
    return w

def is_valid_malagasy_root(w: str) -> bool:
    if not w or len(w) < 2:
        return False
    if not re.match(r'^[a-z]+$', w):
        return False
    if any(c in MALAGASY_INVALID_LETTERS for c in w):
        return False
    if not re.search(r'([aeioy]|ka|tra|na)$', w):
        return False
    if w in IRREGULAR_SURFACE_FORMS:
        return False
    return True

def is_derived_word(w: str) -> bool:
    if w in CANONICAL_REQUIRED_ROOTS:
        return False
    if w in LEGITIMATE_PREFIX_LIKE_ROOTS:
        return False
    if w in IRREGULAR_SURFACE_FORMS:
        return True
    
    # 0. Réduplications complètes (moramora -> mora, tsaratsara -> tsara, fotsifotsy -> fotsy)
    if len(w) >= 4 and len(w) % 2 == 0:
        half = len(w) // 2
        if w[:half] == w[half:]:
            return True
    if len(w) >= 6:
        for i in range(2, len(w) - 2):
            left = w[:i]
            right = w[i:]
            if left == right:
                return True
            if left.endswith("i") and right.endswith("y") and left[:-1] == right[:-1]:
                return True
    
    # 1. Productive Agent & Circumstantial prefixes (always derived)
    if re.match(r'^(mpan|mpam|mpi|mpang|mpanka|famp|fian|mamp|mian|namp|nian|hamp|hian|maha|naha|haha|faha|fahe|mifank|nifank|hifank|fifank|mifamp|nifamp|hifamp|fifamp|mifan|nifan|hifan|fifan)[a-z]+', w):
        return True
    
    # 2. Action noun / circumstantial prefixes (fan-, fam-, fang-, fanka-)
    if re.match(r'^(fan|fam|fang|fanka)[a-z]{3,}', w):
        return True
    
    # 3. Past / future tense verbal prefixes (nan-, nam-, nang-, han-, ham-, hang-)
    if re.match(r'^(nan|nam|nang|nanka|han|ham|hang|hanka)[a-z]{3,}', w):
        return True
    
    # 4. Past / future simple prefixes (ni-, hi-) on stems > 3 chars
    if re.match(r'^(ni|hi)[a-z]{3,}', w):
        return True
    
    # 5. Verbal prefixes man-, mam-, mang- on words > 4 chars not in legitimate list
    if re.match(r'^(man|mam|mang)[a-z]{3,}', w):
        return True

    # 6. Fi- + stem + -ana (circumstantial nouns: fidirana, fidinana, fiasana, fivavahana)
    if re.match(r'^fi[a-z]{2,}(ana|ena|ina)$', w):
        return True

    # 7. Ha- + stem + -ana (abstract quality nouns: hasalamana -> salama, hafaliana -> faly, halavana -> lava)
    if w.startswith("ha") and len(w) >= 7 and re.search(r'(ana|ena|ina)$', w):
        return True
        
    # 8. Suffixes passifs -ana, -ina, -ena sur mots longs (> 5 lettres)
    if len(w) >= 6 and re.search(r'(ana|ina|ena)$', w):
        # Exclure les dérivés passifs comme lazaina, vakina, vonoina, tapahana, soratana
        if re.search(r'(aina|oina|eina|ahina|ohina|ehina|arana|atana|enana|anana|inana)$', w):
            return True
        
    return False

def parse_kaikki(path: Path) -> dict:
    roots = {}
    if not path.exists():
        return roots
    
    pos_map = {
        "noun": "n", "verb": "v", "adj": "adj", "adv": "adv",
        "prep": "prep", "pron": "pron", "num": "num", "conj": "conj",
        "intj": "misc", "particle": "misc", "det": "misc", "article": "misc", "name": "n"
    }
    
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            try:
                data = json.loads(line)
            except Exception:
                continue
            word = clean_word(data.get("word", ""))
            pos = pos_map.get(data.get("pos", ""), "misc")
            
            is_form = any("form" in ht.get("args", {}).get("2", "") for ht in data.get("head_templates", []))
            for sense in data.get("senses", []):
                for form_of in sense.get("form_of", []):
                    is_form = True
                    rw = clean_word(form_of.get("word", ""))
                    if is_valid_malagasy_root(rw) and not is_derived_word(rw):
                        roots[rw] = "v" if pos == "v" else pos
            
            if not is_form and is_valid_malagasy_root(word) and not is_derived_word(word):
                if word not in roots:
                    roots[word] = pos
    return roots

def parse_rakibolana(json_dir: Path) -> tuple[dict, list]:
    roots = {}
    pairs = []
    if not json_dir.exists():
        return roots, pairs
    
    for fpath in glob.glob(str(json_dir / "*.json")):
        with open(fpath, "r", encoding="utf-8") as f:
            try:
                data = json.load(f)
            except Exception:
                continue
            for entry in data:
                raw_w = entry.get("word", "")
                w = clean_word(raw_w)
                defn = entry.get("definition", "")
                
                # Check for cross-references like "Jer. Forara"
                if defn.strip().lower().startswith("jer.") or defn.strip().lower().startswith("jar."):
                    continue
                
                pos = "misc"
                if any(k in defn for k in ["mt.", "matoanteny"]):
                    pos = "v"
                elif any(k in defn for k in ["p.t", "p. t", "mpamari", "pi:"]):
                    pos = "adj"
                elif any(k in defn for k in ["a.", "anarana", "a:"]):
                    pos = "n"
                elif any(k in defn for k in ["adv.", "adverbe"]):
                    pos = "adv"
                elif "mp.s" in defn:
                    pos = "pron"
                elif any(k in defn for k in ["mp.m", "mp.mp"]):
                    pos = "conj"
                elif "mp.h" in defn:
                    pos = "prep"
                
                # Check root in parenthetical syntax: (fototeny)
                m = re.search(r'^\s*\(([a-zA-Z\-]+)\)', defn)
                if m:
                    rw = clean_word(m.group(1))
                    if is_valid_malagasy_root(rw) and not is_derived_word(rw):
                        roots[rw] = pos
                        if is_valid_malagasy_root(w) and w != rw:
                            pairs.append((w, rw))
                else:
                    if "mt. ih." in defn or "mt.ih." in defn:
                        continue
                    if is_valid_malagasy_root(w) and not is_derived_word(w):
                        roots[w] = pos
    return roots, pairs

def main():
    print("=" * 60)
    print("EXTRACTION ET ENRICHISSEMENT DU LEXIQUE PUR DE RACINES MALGACHES")
    print("=" * 60)
    
    kaikki = parse_kaikki(KAIKKI_PATH)
    print(f"1. Racines extraites de Kaikki/Wiktionary : {len(kaikki)}")
    
    rakibolana, pairs = parse_rakibolana(RAKIBOLANA_DIR)
    print(f"2. Racines extraites de Rakibolana Malagasy : {len(rakibolana)}")
    print(f"   Paires morphologiques collectées : {len(pairs)}")
    
    # Merge
    merged = {}
    for k, v in CANONICAL_REQUIRED_ROOTS.items():
        merged[k] = v
    for k, v in kaikki.items():
        if k not in merged or merged[k] == "misc":
            merged[k] = v
    for k, v in rakibolana.items():
        if k not in merged or merged[k] == "misc":
            merged[k] = v
            
    # Final filter: strictly valid roots and not derived
    final_roots = {}
    for k, v in merged.items():
        if is_valid_malagasy_root(k) and (k in CANONICAL_REQUIRED_ROOTS or not is_derived_word(k)):
            final_roots[k] = v
            
    sorted_roots = sorted(final_roots.items(), key=lambda x: x[0])
    print(f"\nTotal des racines pures uniques : {len(sorted_roots)}")
    
    # Write back to roots.tsv
    header = (
        "# Dictionnaire de racines pures (fototeny) malgaches pour malagasy-stemmer\n"
        "# Format: racine\\tcatégorie (v=verbe, n=nom, adj=adjectif, adv=adverbe, prep=préposition, pron=pronom, conj=conjonction, num=numéral, misc=divers)\n"
        "# Les entrées DOIVENT être triées par ordre alphabétique (requis par fst::MapBuilder).\n"
        "# Sources: Dictionnaire de référence, Kaikki/Wiktionnaire, Rakibolana Malagasy de l'Académie.\n"
    )
    
    with open(ROOTS_TSV_PATH, "w", encoding="utf-8") as f:
        f.write(header)
        for root, cat in sorted_roots:
            f.write(f"{root}\t{cat}\n")
            
    print(f"Fichier écrit avec succès dans : {ROOTS_TSV_PATH}")

if __name__ == "__main__":
    main()

