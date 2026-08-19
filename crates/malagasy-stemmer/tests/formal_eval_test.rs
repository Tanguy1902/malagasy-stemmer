use std::collections::HashMap;
use malagasy_stemmer::stem;

struct EvalCase {
    surface: String,
    expected_root: String,
    category: String,
}

fn load_gold_standard() -> Vec<EvalCase> {
    let tsv_content = include_str!("../data/eval_gold_standard.tsv");
    tsv_content
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                Some(EvalCase {
                    surface: parts[0].trim().to_string(),
                    expected_root: parts[1].trim().to_string(),
                    category: parts[2].trim().to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn test_formal_evaluation_accuracy() {
    let cases = load_gold_standard();
    assert!(!cases.is_empty(), "Le dataset d'évaluation ne doit pas être vide");

    let total = cases.len();
    let mut total_correct = 0;
    let mut cat_total: HashMap<String, usize> = HashMap::new();
    let mut cat_correct: HashMap<String, usize> = HashMap::new();

    for case in &cases {
        *cat_total.entry(case.category.clone()).or_insert(0) += 1;
        let actual = stem(&case.surface);
        if actual == case.expected_root {
            total_correct += 1;
            *cat_correct.entry(case.category.clone()).or_insert(0) += 1;
        }
    }

    let global_accuracy = (total_correct as f64 / total as f64) * 100.0;

    println!("\n==================================================================");
    println!("RAPPORT D'ÉVALUATION FORMELLE — malagasy-stemmer (Rust Crate)");
    println!("==================================================================");
    println!("{:<22} | {:<12} | {:<18}", "Catégorie", "Échantillons", "Précision Exacte");
    println!("------------------------------------------------------------------");

    let mut sorted_cats: Vec<String> = cat_total.keys().cloned().collect();
    sorted_cats.sort();

    for cat in &sorted_cats {
        let tot = cat_total[cat];
        let corr = cat_correct.get(cat).copied().unwrap_or(0);
        let acc = (corr as f64 / tot as f64) * 100.0;
        println!("{:<22} | {:<12} | {:>6.2}% ({}/{})", cat, tot, acc, corr, tot);
    }

    println!("------------------------------------------------------------------");
    println!(
        "{:<22} | {:<12} | {:>6.2}% ({}/{})",
        "GLOBAL SCORE", total, global_accuracy, total_correct, total
    );
    println!("==================================================================\n");

    // Garantir un score d'Exact Match minimum sur le dataset d'évaluation
    assert!(
        global_accuracy >= 65.0,
        "La précision globale ({:.2}%) doit être au moins de 65%",
        global_accuracy
    );
}
