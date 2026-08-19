"""Tests unitaires pour l'API Python de malagasy-stemmer."""

import pytest
import malagasy_stemmer as mg


def test_stem_individual_words():
    # Verbes actifs (man-, mam-, mang-)
    assert mg.stem("manoratra") == "soratra"
    assert mg.stem("nanoratra") == "soratra"
    assert mg.stem("hanoratra") == "soratra"
    assert mg.stem("mamaky") == "vaky"
    assert mg.stem("mamboly") == "voly"
    assert mg.stem("mangalatra") == "halatra"

    # Passifs & dérivés nominaux
    assert mg.stem("soratana") == "soratra"
    assert mg.stem("vakina") == "vaky"
    assert mg.stem("mpanoratra") == "soratra"
    assert mg.stem("fanoratana") == "soratra"
    assert mg.stem("fampianarana") == "anatra"
    assert mg.stem("mpianatra") == "anatra"
    assert mg.stem("fandrosoana") == "roso"
    assert mg.stem("fanjakana") == "zaka"
    assert mg.stem("fanisana") == "isa"
    assert mg.stem("fahendrena") == "hendry"
    assert mg.stem("fahasalamana") == "salama"

    # Réduplication
    assert mg.stem("tsaratsara") == "tsara"
    assert mg.stem("moramora") == "mora"

    # Mots composés et Sandhi
    assert mg.stem("harem-pirenena") == "harena_firenena"
    assert mg.stem("tanan-dehibe") == "tanana_lehibe"
    assert mg.stem("ara-potoana") == "araka_fotoana"

    # Alternances vocaliques & passifs directs
    assert mg.stem("tenenina") == "teny"
    assert mg.stem("tsindriana") == "tsindry"
    assert mg.stem("lazaina") == "laza"
    assert mg.stem("fidina") == "fidy"

    # Formes impératives
    assert mg.stem("soraty") == "soratra"
    assert mg.stem("vakio") == "vaky"
    assert mg.stem("tapaho") == "tapaka"
    assert mg.stem("fidio") == "fidy"

    # Formes irrégulières & supplétives (Phase 2)
    assert mg.stem("mandeha") == "leha"
    assert mg.stem("nandeha") == "leha"
    assert mg.stem("handeha") == "leha"
    assert mg.stem("fandehanana") == "leha"
    assert mg.stem("homana") == "hano"
    assert mg.stem("mihinana") == "hano"
    assert mg.stem("hanina") == "hano"
    assert mg.stem("nohanina") == "hano"
    assert mg.stem("entina") == "tondra"
    assert mg.stem("nentina") == "tondra"
    assert mg.stem("maka") == "aka"
    assert mg.stem("alaina") == "aka"
    assert mg.stem("manome") == "ome"
    assert mg.stem("omena") == "ome"
    assert mg.stem("amidy") == "varotra"
    assert mg.stem("misy") == "misy"
    assert mg.stem("nisy") == "misy"
    assert mg.stem("mandre") == "re"
    assert mg.stem("mamoaka") == "voaka"
    assert mg.stem("miditra") == "iditra"
    assert mg.stem("ampidiro") == "iditra"
    assert mg.stem("miakatra") == "akatra"
    assert mg.stem("midina") == "dina"


def test_stem_with_details():
    res = mg.stem_with_details("manoratra")
    assert isinstance(res, mg.StemResult)
    assert res.original == "manoratra"
    assert res.root == "soratra"
    assert res.in_dictionary is True
    assert res.confidence > 0.8
    assert "prefix" in res.operation


def test_stemmer_class_batch():
    stemmer = mg.MalagasyStemmer()
    words = ["manoratra", "fampianarana", "tsaratsara", "harem-pirenena"]
    roots = stemmer.stem_batch(words)
    assert roots == ["soratra", "anatra", "tsara", "harena_firenena"]

    details = stemmer.stem_batch_with_details(words)
    assert len(details) == 4
    assert details[0].root == "soratra"
    assert details[1].root == "anatra"


def test_tokenize_and_stem():
    text = "Nanoratra taratasy momba ny fampianarana sy ny fambolena ary ny harem-pirenena tamin'ny solosaina izy ireo."
    roots = mg.tokenize_and_stem(text, remove_stopwords=True)

    # Vérifier que les stopwords ont été retirés
    assert "ny" not in roots
    assert "sy" not in roots
    assert "momba" not in roots
    assert "ary" not in roots

    # Vérifier que les racines extraites sont présentes
    assert "soratra" in roots
    assert "taratasy" in roots
    assert "anatra" in roots
    assert "voly" in roots
    assert "harena_firenena" in roots
    assert "solosaina" in roots


def test_stopwords():
    assert mg.is_stopword("ny") is True
    assert mg.is_stopword("dia") is True
    assert mg.is_stopword("sy") is True
    assert mg.is_stopword("soratra") is False
    assert mg.is_stopword("trano") is False


def test_fuzzy_lookup():
    matches = mg.fuzzy_root_lookup("sorata", max_distance=1)
    assert len(matches) > 0
    assert matches[0].word == "soratra"
    assert matches[0].distance == 1

