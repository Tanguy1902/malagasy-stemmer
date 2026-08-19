use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("roots.fst");

    // Lire les racines depuis le fichier TSV
    let tsv_path = Path::new("data/roots.tsv");
    let tsv_content = fs::read_to_string(tsv_path)
        .unwrap_or_else(|e| panic!("Impossible de lire {}: {}", tsv_path.display(), e));

    // Parser les racines (ignorer commentaires et lignes vides, prendre la première colonne)
    let mut roots: Vec<String> = tsv_content
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .filter_map(|line| {
            let root = line.split('\t').next()?.trim().to_lowercase();
            if root.is_empty() { None } else { Some(root) }
        })
        .collect();

    // Dédupliquer et trier (OBLIGATOIRE pour fst::SetBuilder)
    roots.sort();
    roots.dedup();

    // Construire le FST (Finite State Transducer)
    let mut builder = fst::SetBuilder::memory();
    for root in &roots {
        builder.insert(root.as_bytes()).unwrap_or_else(|e| {
            panic!("Erreur lors de l'insertion de '{}' dans le FST : {}", root, e);
        });
    }
    let fst_bytes = builder.into_inner().unwrap_or_else(|e| {
        panic!("Erreur lors de la finalisation du FST : {}", e);
    });

    // Écrire le FST compilé dans OUT_DIR
    fs::write(&dest_path, &fst_bytes).unwrap_or_else(|e| {
        panic!("Erreur lors de l'écriture du FST : {}", e);
    });

    // Recompiler uniquement si le fichier de données change
    println!("cargo:rerun-if-changed=data/roots.tsv");
    println!(
        "cargo:warning=malagasy-stemmer: Compiled {} unique roots into FST ({} bytes)",
        roots.len(),
        fst_bytes.len()
    );
}
