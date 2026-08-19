# Théorie Linguistique & Morphologie Malgache

La langue malgache (*Teny Malagasy*) appartient à la branche malayo-polynésienne de la famille des **langues austronésiennes**. C'est une langue hautement **agglutinante** dont le système de dérivation repose sur des radicaux ou racines (*fototeny*) modifiés par des affixes (préfixes, suffixes, infixes, circonfixes) et des mutations phonétiques régulières.

`malagasy-stemmer` modélise ces règles linguistiques formelles à l'aide d'une approche à deux niveaux (*Two-Level Morphology*).

---

## 1. Mutations Nasales (*Fiovan-drenifeo an-tsona*)

Les préfixes verbaux actifs (`man-`, `nan-`, `han-`, `fan-`, `mpan-`, `mam-`, `mang-`) déclenchent une mutation de la consonne initiale de la racine.

### Table des correspondances nasales

| Préfixe de surface | Consonne sous-jacente restituée | Exemples de formes fléchies | Racine canonique (*Fototeny*) |
| :--- | :--- | :--- | :--- |
| **`man-`** + voyelle | **`s`**, **`t`**, **`ts`** | `manoratra`<br>`manapaka`<br>`manindry` | `soratra`<br>`tapaka`<br>`tsindry` |
| **`man-`** + `d` | **`l`**, **`t`**, **`d`** | `mandatsaka`<br>`mandoka`<br>`mandidy` | `latsaka`<br>`toka`<br>`didy` |
| **`man-`** + `dr` | **`r`** | `mandroso` | `roso` |
| **`man-`** + `j` | **`z`**, **`j`** | `manjaitra`<br>`manjery` | `zaitra`<br>`jery` |
| **`mam-`** + voyelle | **`v`**, **`p`**, **`b`** | `mamaky`<br>`mametraka`<br>`mamabo` | `vaky`<br>`petraka`<br>`babo` |
| **`mam-`** + `b` | **`v`**, **`b`** | `mamboly`<br>`mpambotry` | `voly`<br>`botry` |
| **`mam-`** + `p` | **`f`** | `mampiditra` | `iditra` |
| **`mang-`** + voyelle | **`h`**, **`k`** | `mangalatra`<br>`mangady` | `halatra`<br>`kady` |

---

## 2. Suffixes & Restauration Morphophonémique

Les suffixes passifs (`-ina`, `-ana`, `-ena`, `-ona`) et les désinences de mode (impératif `-y`, `-o`) provoquent souvent une contraction ou une modification de la finale de la racine.

### Règles d'alternance des finales

1. **Restauration de `-tra`** :
   - Lorsque la forme suffixée contient un `t` ou un `r` intermédiaire :
   - `soratana` $\rightarrow$ base `sorat` $\rightarrow$ restitution : **`soratra`**
   - `anarana` $\rightarrow$ base `anar` $\rightarrow$ restitution : **`anatra`**

2. **Restauration de `-ka`** :
   - Lorsque la consonne finale est affaiblie en `h` :
   - `tapahana` $\rightarrow$ base `tapah` $\rightarrow$ restitution : **`tapaka`**
   - `tapaho` (impératif) $\rightarrow$ restitution : **`tapaka`**

3. **Alternance vocalique `y` $\leftrightarrow$ `i` / `e`** :
   - En malgache, la lettre `y` n'apparaît qu'en fin de mot ; à l'intérieur d'un mot dérivé, elle se transforme en `i` ou `e` :
   - `vakina` $\rightarrow$ base `vak` $\rightarrow$ restitution : **`vaky`**
   - `tenenina` $\rightarrow$ base `tenen` $\rightarrow$ restitution : **`teny`**
   - `tsindriana` $\rightarrow$ base `tsindri` $\rightarrow$ restitution : **`tsindry`**

---

## 3. Infixes Aspectuels (`-in-`, `-om-`)

L'infixe s'insère directement après la première consonne de la racine :

- **Infixe passif / perfectif `-in-`** :
  - $C + \text{-in-} + \text{reste}$ $\rightarrow$ $C + \text{reste}$
  - `vinaky` $\rightarrow$ racine : **`vaky`**
  - `tinapaka` $\rightarrow$ racine : **`tapaka`**
  - `jinery` $\rightarrow$ racine : **`jery`**

- **Infixe statif / potentiel `-om-`** :
  - `tomanany` $\rightarrow$ racine : **`tanana`**

---

## 4. Réduplication (*Famerenana Fototeny*)

La réduplication (totale ou partielle) est un procédé morphologique fréquent en malgache, exprimant l'atténuation, l'itération ou l'intensification :

- **Réduplication exacte** :
  - `moramora` (doucement) $\rightarrow$ racine : **`mora`** (facile / doux)
  - `tsaratsara` (assez bon) $\rightarrow$ racine : **`tsara`** (bon / bien)
  - `kelykely` (petit à petit) $\rightarrow$ racine : **`kely`** (petit)

- **Réduplication avec liaison consonantique** :
  - `haingankaingana` $\rightarrow$ racine : **`haingana`**

---

## 5. Mots Composés & Sandhi Consonantique

Dans les mots composés reliés par un tiret, la première racine subit souvent une apocope nasale (`-m`, `-n`) et la consonne initiale de la seconde racine subit une mutation de contact (*Sandhi*) :

| Mot composé de surface | Rétablissement Sandhi | Racines canoniques extraites |
| :--- | :--- | :--- |
| `harem-pirenena` | `harena` + (`p` $\rightarrow$ `f`) `firenena` | **`harena_firenena`** |
| `tanan-dehibe` | `tanana` + (`d` $\rightarrow$ `l`) `lehibe` | **`tanana_lehibe`** |
| `ara-potoana` | `araka` + (`p` $\rightarrow$ `f`) `fotoana` | **`araka_fotoana`** |
| `ara-toekarena` | `araka` + `toekarena` | **`araka_toekarena`** |

---

## 6. Formes Irrégulières & Supplétives

Certains verbes très fréquents possèdent des radicaux supplétifs historiques :

- `mandeha` / `nandeha` / `fandehanana` $\rightarrow$ racine : **`leha`**
- `homana` / `mihinana` / `hanina` / `fihinanana` $\rightarrow$ racine : **`hano`**
- `entina` / `nentina` / `ento` $\rightarrow$ racine : **`tondra`**
- `alaina` / `maka` / `nalaina` / `alao` $\rightarrow$ racine : **`aka`**
- `manome` / `omena` / `omeo` $\rightarrow$ racine : **`ome`**
- `amidy` / `namidy` / `famidiana` $\rightarrow$ racine : **`varotra`**
