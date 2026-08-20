#!/usr/bin/env python3
"""
Exemple de projet RAG simple pour des documents en langue malgache.
Utilise malagasy-stemmer pour la normalisation morphologique et BM25 pour la recherche.
"""

import math
from collections import Counter
from dataclasses import dataclass
import malagasy_stemmer as mg


# 1. Corpus de documents de test en malgache
KNOWLEDGE_BASE = [
    {
        "id": "doc_1",
        "title": "Fambolena Lavanila sy Jirofo",
        "content": (
            "Madagasikara dia anisan'ny firenena mpamokatra lavanila sy jirofo lehibe indrindra maneran-tany. "
            "Ny fambolena lavanila any amin'ny faritra Sava dia manome fidiram-bola lehibe ho an'ny tantsaha "
            "sy ny toekarem-pirenena. Mamboly sy mikarakara tsara ny tany izy ireo mba hahazoana vokatra tsara."
        ),
    },
    {
        "id": "doc_2",
        "title": "Ny Tantaran'ny Fampianarana",
        "content": (
            "Nanomboka tamin'ny taonjato fahasivy ambin'ny folo ny fampianarana an-tsekoly maoderina teto Madagasikara. "
            "Nanoratra boky sy taratasy maro ny mpampianatra mba hianaran'ny mpianatra mamaky teny sy manoratra. "
            "Ankehitriny dia maro ny sekoly sy oniversite manerana ny Nosy."
        ),
    },
    {
        "id": "doc_3",
        "title": "Fiarovana ny Tontolo Iainana sy ny Harem-pirenena",
        "content": (
            "Manankarena tokoa i Madagasikara amin'ny lafiny biby sy zavamaniry tsy hita any an-kafa. "
            "Ilaina ny miaro ny ala sy ny harem-pirenena amin'ny doro tanety sy ny fandripahana hazo tsy ara-dalàna. "
            "Mampianatra ny vahoaka momba ny fiarovana ny zavaboary ny fikambanana maro."
        ),
    },
    {
        "id": "doc_4",
        "title": "Fiompiana sy Fambolena Vary",
        "content": (
            "Ny vary no foto-tsakafon'ny Malagasy. Mamboly vary an-tanimbary ny ankamaroan'ny mponina any ambanivohitra. "
            "Mpiompy omby sy kisoa koa ny tantsaha mba hanampiana ny fidiram-bolan'ny tokantrano."
        ),
    },
]


@dataclass
class SearchResult:
    doc_id: str
    title: str
    content: str
    score: float
    matched_roots: list[str]


class SimpleMalagasyRAG:
    """Moteur RAG simplifié avec indexation morphologique malgache."""

    def __init__(self, k1: float = 1.5, b: float = 0.75):
        self.k1 = k1
        self.b = b
        self.documents = []
        self.doc_lengths = []
        self.avg_doc_len = 0.0
        self.inverted_index = {}
        self.doc_term_freqs = []

    def index_documents(self, docs: list[dict]):
        """Indexe les documents en extrayant les racines morphologiques."""
        self.documents = docs
        self.doc_lengths = []
        self.doc_term_freqs = []
        self.inverted_index = {}

        total_len = 0
        for doc_idx, doc in enumerate(docs):
            # Normalisation complète : tokenisation + retrait stopwords + stemming morphologique
            roots = mg.tokenize_and_stem(doc["content"], remove_stopwords=True)
            doc_len = len(roots)
            self.doc_lengths.append(doc_len)
            total_len += doc_len

            term_counts = Counter(roots)
            self.doc_term_freqs.append(term_counts)

            for term in term_counts:
                if term not in self.inverted_index:
                    self.inverted_index[term] = []
                self.inverted_index[term].append(doc_idx)

        self.avg_doc_len = total_len / len(docs) if docs else 1.0

    def search(self, query: str, top_k: int = 2) -> list[SearchResult]:
        """Recherche les documents pertinents pour une requête en malgache."""
        query_roots = mg.tokenize_and_stem(query, remove_stopwords=True)
        if not query_roots:
            return []

        scores = [0.0] * len(self.documents)
        n_docs = len(self.documents)

        for term in query_roots:
            if term not in self.inverted_index:
                continue

            posting_list = self.inverted_index[term]
            df = len(posting_list)
            # Calcul IDF (Inverse Document Frequency)
            idf = math.log(1 + (n_docs - df + 0.5) / (df + 0.5))

            for doc_idx in posting_list:
                tf = self.doc_term_freqs[doc_idx][term]
                doc_len = self.doc_lengths[doc_idx]

                # Formule BM25
                numerator = tf * (self.k1 + 1)
                denominator = tf + self.k1 * (1 - self.b + self.b * (doc_len / self.avg_doc_len))
                scores[doc_idx] += idf * (numerator / denominator)

        # Trier par score décroissant
        ranked_indices = sorted(range(len(scores)), key=lambda i: scores[i], reverse=True)

        results = []
        for idx in ranked_indices[:top_k]:
            if scores[idx] > 0.0:
                doc = self.documents[idx]
                matched = [term for term in query_roots if term in self.doc_term_freqs[idx]]
                results.append(
                    SearchResult(
                        doc_id=doc["id"],
                        title=doc["title"],
                        content=doc["content"],
                        score=round(scores[idx], 4),
                        matched_roots=matched,
                    )
                )

        return results

    def answer_question(self, query: str) -> str:
        """Pipeline RAG complet : Récupération de contexte + Génération."""
        print(f"\nFANONTANIANA (Question) : « {query} »")
        
        # Décomposition morphologique de la requête
        query_roots = mg.tokenize_and_stem(query, remove_stopwords=True)
        print(f"Racines extraites de la requête : {query_roots}")

        results = self.search(query, top_k=2)

        if not results:
            return "Tsy nahitana valiny mifanaraka amin'ny fanontaniana."

        print("\nTAHIRIN-KEVITRA HITA (Documents Récupérés) :")
        for i, res in enumerate(results, 1):
            print(f"  [{i}] {res.title} (Score BM25: {res.score})")
            print(f"      Racines correspondantes : {res.matched_roots}")
            print(f"      Extrait : {res.content[:140]}...")

        # Contexte synthétisé pour l'étape de génération LLM
        context = "\n".join([f"- {r.content}" for r in results])
        
        return (
            f"\nVALINTENY SY FAMINTINANA (Réponse RAG) :\n"
            f"Araka ny tahirin-kevitra, mifototra amin'ireto fampahalalana ireto ny valiny :\n"
            f"{context}"
        )


def main():
    rag = SimpleMalagasyRAG()
    print("Fampidirana ireo tahirin-kevitra (Indexation en cours)...")
    rag.index_documents(KNOWLEDGE_BASE)
    print(f"Documents indexés avec succès : {len(KNOWLEDGE_BASE)} documents.\n")

    # Tests de requêtes avec des flexions verbales très différentes du document source
    queries = [
        # Test 1 : 'Te hianatra' (verbe) doit matcher 'fampianarana' / 'mpianatra' -> racine 'anatra'
        "Te hianatra momba ny tantaran'ny sekoly aho",
        
        # Test 2 : 'Ahoana ny fambolena' doit matcher 'mamboly' / 'lavanila' -> racine 'voly'
        "Ahoana ny fambolena sy ny vokatra lavanila any Sava ?",

        # Test 3 : 'Fiarovana ny ala' doit matcher 'harem-pirenena' -> racine 'araka' / 'harena_firenena'
        "Inona ny fomba fiarovana ny harem-pirenena sy ny biby ?",
    ]

    for q in queries:
        answer = rag.answer_question(q)
        print(answer)
        print("-" * 75)


if __name__ == "__main__":
    main()
