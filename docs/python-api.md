# Référence de l'API Python

Le package Python `malagasy-stemmer` expose des fonctions rapides au niveau du module ainsi qu'une classe optimisée `MalagasyStemmer` pour le traitement par lot.

---

## Fonctions du Module

### `stem(word: str) -> str`

Extrait la racine canonique (_fototeny_) d'un mot malgache isolé.

```python
import malagasy_stemmer as mg

root = mg.stem("manoratra")
print(root)  # "soratra"
```

---

### `stem_with_details(word: str) -> StemResult`

Extrait la racine et retourne l'ensemble des métadonnées morphologiques (score de confiance, type d'opération morphologique appliquée, présence dans le dictionnaire de référence).

```python
res = mg.stem_with_details("fampianarana")

print(res.original)       # "fampianarana"
print(res.root)           # "anatra"
print(res.confidence)     # 0.87 (score entre 0.0 et 1.0)
print(res.operation)      # "prefix_then_suffix"
print(res.in_dictionary)  # True
```

---

### `tokenize(text: str) -> list[str]`

Découpe un texte malgache en tokens élémentaires en préservant la casse minuscule, en gérant la typographie Unicode (apostrophes courbes `’`, tirets cadratins `—`) et en découpant intelligemment les contractions malgaches (`amin'ny`, `an'i`, etc.).

```python
tokens = mg.tokenize("Niresaka tamin'ny mpianatra izy.")
print(tokens)
# ['niresaka', 'tamin', 'ny', 'mpianatra', 'izy']
```

---

### `tokenize_and_stem(text: str, remove_stopwords: bool = True) -> list[str]`

Pipeline complet combinant tokenisation, filtrage optionnel des mots vides (_stopwords_) et extraction morphologique des racines.

```python
text = "Nanoratra taratasy momba ny fampianarana sy ny fambolena izy."
roots = mg.tokenize_and_stem(text, remove_stopwords=True)
print(roots)
# ['soratra', 'taratasy', 'anatra', 'voly']
```

---

### `tokenize_and_stem_with_details(text: str, remove_stopwords: bool = True) -> list[StemResult]`

Similaire à `tokenize_and_stem`, mais retourne une liste d'objets `StemResult` complets pour chaque token non filtré.

```python
results = mg.tokenize_and_stem_with_details("Mamboly vary ny tantsaha.", remove_stopwords=True)
for r in results:
    print(f"{r.original:10} -> {r.root:10} [{r.operation}]")
```

---

### `is_stopword(word: str) -> bool`

Vérifie si un mot donné fait partie de la liste des mots vides malgaches courants (articles `ny`, `ilay`, pronoms `izy`, `isika`, conjonctions `sy`, `ary`, `fa`, prépositions `amin`, adverbes `dia`, `koa`, etc.).

```python
print(mg.is_stopword("ny"))       # True
print(mg.is_stopword("dia"))      # True
print(mg.is_stopword("soratra"))  # False
```

---

### `fuzzy_root_lookup(word: str, max_distance: int = 1) -> list[FuzzyMatch]`

Recherche les racines les plus proches dans le dictionnaire FST selon la distance de Levenshtein. Idéal pour la correction automatique de fautes de frappe.

```python
# Recherche avec 1 faute de frappe :
matches = mg.fuzzy_root_lookup("sorata", max_distance=1)
for m in matches:
    print(f"Racine trouvée : {m.word} (distance = {m.distance})")
# Racine trouvée : soratra (distance = 1)
```

---

## Classe `MalagasyStemmer`

La classe `MalagasyStemmer` encapsule une instance réutilisable du moteur morphologique pour un traitement par lot à haut débit.

```python
stemmer = mg.MalagasyStemmer()
```

### Méthodes :

- `stem(word: str) -> str`
- `stem_with_details(word: str) -> StemResult`
- `stem_batch(words: list[str]) -> list[str]`
- `stem_batch_with_details(words: list[str]) -> list[StemResult]`

#### Exemple Batch :

```python
words = ["manoratra", "fampianarana", "tsaratsara", "harem-pirenena", "mamaky"]
roots = stemmer.stem_batch(words)
print(roots)
# ['soratra', 'anatra', 'tsara', 'harena_firenena', 'vaky']
```

---

## Structures de données

### `StemResult`

| Attribut        | Type    | Description                                                   |
| :-------------- | :------ | :------------------------------------------------------------ |
| `original`      | `str`   | Le mot d'origine fourni en entrée.                            |
| `root`          | `str`   | La racine canonique extraite (_fototeny_).                    |
| `confidence`    | `float` | Score de confiance probabiliste de 0.0 à 1.0.                 |
| `operation`     | `str`   | Nom de l'opération morphologique déterminante.                |
| `in_dictionary` | `bool`  | `True` si la racine extraite figure dans le dictionnaire FST. |

### `FuzzyMatch`

| Attribut   | Type  | Description                                                               |
| :--------- | :---- | :------------------------------------------------------------------------ |
| `word`     | `str` | La racine candidate trouvée dans le dictionnaire.                         |
| `distance` | `int` | Distance de Levenshtein (nombre d'insertions/suppressions/substitutions). |
