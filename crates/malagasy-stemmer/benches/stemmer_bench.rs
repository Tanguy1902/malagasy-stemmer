use criterion::{black_box, criterion_group, criterion_main, Criterion};
use malagasy_stemmer::{stem, tokenize_and_stem, MalagasyStemmer};

fn bench_stem_individual(c: &mut Criterion) {
    let words = [
        "manoratra",
        "nanoratra",
        "fampianarana",
        "tsaratsara",
        "harem-pirenena",
        "mamboly",
        "mianatra",
        "mpanoratra",
        "mamaky",
        "moramora",
    ];

    c.bench_function("stem_individual_10_words", |b| {
        b.iter(|| {
            for &w in &words {
                let _ = stem(black_box(w));
            }
        })
    });
}

fn bench_stem_batch(c: &mut Criterion) {
    let stemmer = MalagasyStemmer::new();
    let words: Vec<&str> = (0..1000)
        .map(|i| match i % 5 {
            0 => "manoratra",
            1 => "fampianarana",
            2 => "mamboly",
            3 => "tsaratsara",
            _ => "harem-pirenena",
        })
        .collect();

    c.bench_function("stem_batch_1000_words", |b| {
        b.iter(|| {
            let _ = stemmer.stem_batch(black_box(&words));
        })
    });
}

fn bench_tokenize_and_stem_paragraph(c: &mut Criterion) {
    let paragraph = "Ny teny malagasy dia manankarena tokoa amin'ny lafiny voambolana sy rafitra ara-pitsipika. \
        Amin'ny alalan'ny rafitra haingana dia afaka manao fanadihadiana lalina momba ny fampianarana sy ny \
        fambolena eto Madagasikara isika.";

    c.bench_function("tokenize_and_stem_paragraph", |b| {
        b.iter(|| {
            let _ = tokenize_and_stem(black_box(paragraph), true);
        })
    });
}

criterion_group!(
    benches,
    bench_stem_individual,
    bench_stem_batch,
    bench_tokenize_and_stem_paragraph
);
criterion_main!(benches);
