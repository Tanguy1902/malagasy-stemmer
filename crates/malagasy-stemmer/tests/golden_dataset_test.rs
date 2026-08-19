
use malagasy_stemmer::{stem, tokenize_and_stem};

/// Structure de cas de test morphologique
struct TestCase {
    surface_form: &'static str,
    expected_root: &'static str,
    category: &'static str,
}

const GOLDEN_TESTSET: &[TestCase] = &[
    // 1. Mutations nasales - Temps et voix active (man-, nan-, han-, fan-, mpan-)
    TestCase { surface_form: "manoratra", expected_root: "soratra", category: "nasal_active_present" },
    TestCase { surface_form: "nanoratra", expected_root: "soratra", category: "nasal_active_past" },
    TestCase { surface_form: "hanoratra", expected_root: "soratra", category: "nasal_active_future" },
    TestCase { surface_form: "fanoratana", expected_root: "soratra", category: "nasal_nominal_circumstantial" },
    TestCase { surface_form: "mpanoratra", expected_root: "soratra", category: "nasal_agent_noun" },
    
    TestCase { surface_form: "mamaky", expected_root: "vaky", category: "nasal_labial_present" },
    TestCase { surface_form: "namaky", expected_root: "vaky", category: "nasal_labial_past" },
    TestCase { surface_form: "hamaky", expected_root: "vaky", category: "nasal_labial_future" },
    TestCase { surface_form: "mpamaky", expected_root: "vaky", category: "nasal_labial_agent" },

    TestCase { surface_form: "mamboly", expected_root: "voly", category: "nasal_labial_voly" },
    TestCase { surface_form: "namboly", expected_root: "voly", category: "nasal_labial_past_voly" },
    TestCase { surface_form: "fambolena", expected_root: "voly", category: "nasal_labial_circumstantial_voly" },
    TestCase { surface_form: "mpamboly", expected_root: "voly", category: "nasal_labial_agent_voly" },

    TestCase { surface_form: "mangalatra", expected_root: "halatra", category: "nasal_velar_present" },
    TestCase { surface_form: "nangalatra", expected_root: "halatra", category: "nasal_velar_past" },
    TestCase { surface_form: "fangalatra", expected_root: "halatra", category: "nasal_velar_circumstantial" },

    TestCase { surface_form: "mandatsaka", expected_root: "latsaka", category: "nasal_dental_mutation_d_l" },
    TestCase { surface_form: "mandroso", expected_root: "roso", category: "nasal_dental_mutation_dr_r" },
    TestCase { surface_form: "fandrosoana", expected_root: "roso", category: "nasal_circumstantial_dr_r" },
    TestCase { surface_form: "manjaka", expected_root: "zaka", category: "nasal_palatal_mutation_j_z" },
    TestCase { surface_form: "fanjakana", expected_root: "zaka", category: "nasal_circumstantial_j_z" },

    // 2. Préfixes simples & causatifs
    TestCase { surface_form: "mianatra", expected_root: "anatra", category: "prefix_mi" },
    TestCase { surface_form: "mpianatra", expected_root: "anatra", category: "prefix_mpi" },
    TestCase { surface_form: "fampianarana", expected_root: "anatra", category: "prefix_fampi" },
    TestCase { surface_form: "mahafantatra", expected_root: "fantatra", category: "prefix_maha" },
    TestCase { surface_form: "mahasalama", expected_root: "salama", category: "prefix_maha" },
    TestCase { surface_form: "fahasalamana", expected_root: "salama", category: "prefix_faha" },
    TestCase { surface_form: "fahendrena", expected_root: "hendry", category: "prefix_faha_hendry" },
    TestCase { surface_form: "tafapetraka", expected_root: "petraka", category: "prefix_tafa" },

    // 3. Passifs & Suffixes avec restauration morphophonémique
    TestCase { surface_form: "soratana", expected_root: "soratra", category: "passive_tra_restoration" },
    TestCase { surface_form: "vakina", expected_root: "vaky", category: "passive_ina_vaky" },
    TestCase { surface_form: "anarana", expected_root: "anatra", category: "passive_r_to_tra" },
    TestCase { surface_form: "tapahana", expected_root: "tapaka", category: "passive_h_to_ka" },

    // 4. Alternances vocaliques & passifs directs
    TestCase { surface_form: "tenenina", expected_root: "teny", category: "vowel_alternation_y_e" },
    TestCase { surface_form: "tsindriana", expected_root: "tsindry", category: "vowel_alternation_i_y" },
    TestCase { surface_form: "lazaina", expected_root: "laza", category: "direct_passive_lazaina" },
    TestCase { surface_form: "fidina", expected_root: "fidy", category: "passive_fidy" },

    // 5. Formes impératives & optatives
    TestCase { surface_form: "soraty", expected_root: "soratra", category: "imperative_soraty" },
    TestCase { surface_form: "vakio", expected_root: "vaky", category: "imperative_vakio" },
    TestCase { surface_form: "tapaho", expected_root: "tapaka", category: "imperative_tapaho" },
    TestCase { surface_form: "fidio", expected_root: "fidy", category: "imperative_fidio" },

    // 6. Infixes (-in-, -om-)
    TestCase { surface_form: "vinaky", expected_root: "vaky", category: "infix_in_vinaky" },
    TestCase { surface_form: "tinapaka", expected_root: "tapaka", category: "infix_in_tinapaka" },

    // 7. Réduplications
    TestCase { surface_form: "tsaratsara", expected_root: "tsara", category: "reduplication_exact_tsara" },
    TestCase { surface_form: "moramora", expected_root: "mora", category: "reduplication_exact_mora" },

    // 8. Mots composés & Sandhi
    TestCase { surface_form: "harem-pirenena", expected_root: "harena_firenena", category: "compound_sandhi_p_f" },
    TestCase { surface_form: "tanan-dehibe", expected_root: "tanana_lehibe", category: "compound_sandhi_d_l" },
    TestCase { surface_form: "ara-potoana", expected_root: "araka_fotoana", category: "compound_ara_sandhi" },

    // 9. Verbes irréguliers & Formes supplétives (Phase 2)
    TestCase { surface_form: "mandeha", expected_root: "leha", category: "irregular_mandeha_present" },
    TestCase { surface_form: "nandeha", expected_root: "leha", category: "irregular_mandeha_past" },
    TestCase { surface_form: "handeha", expected_root: "leha", category: "irregular_mandeha_future" },
    TestCase { surface_form: "fandehanana", expected_root: "leha", category: "irregular_mandeha_circumstantial" },

    TestCase { surface_form: "homana", expected_root: "hano", category: "irregular_homana" },
    TestCase { surface_form: "mihinana", expected_root: "hano", category: "irregular_mihinana" },
    TestCase { surface_form: "hanina", expected_root: "hano", category: "irregular_hanina_passive" },
    TestCase { surface_form: "nohanina", expected_root: "hano", category: "irregular_nohanina_past_passive" },
    TestCase { surface_form: "fihinanana", expected_root: "hano", category: "irregular_fihinanana_circumstantial" },

    TestCase { surface_form: "entina", expected_root: "tondra", category: "irregular_entina_passive" },
    TestCase { surface_form: "nentina", expected_root: "tondra", category: "irregular_nentina_past_passive" },
    TestCase { surface_form: "ento", expected_root: "tondra", category: "irregular_ento_imperative" },

    TestCase { surface_form: "maka", expected_root: "aka", category: "irregular_maka_active" },
    TestCase { surface_form: "alaina", expected_root: "aka", category: "irregular_alaina_passive" },
    TestCase { surface_form: "nalaina", expected_root: "aka", category: "irregular_nalaina_past_passive" },
    TestCase { surface_form: "alao", expected_root: "aka", category: "irregular_alao_imperative" },

    TestCase { surface_form: "manome", expected_root: "ome", category: "irregular_manome" },
    TestCase { surface_form: "omena", expected_root: "ome", category: "irregular_omena_passive" },
    TestCase { surface_form: "nomena", expected_root: "ome", category: "irregular_nomena_past_passive" },
    TestCase { surface_form: "omeo", expected_root: "ome", category: "irregular_omeo_imperative" },

    TestCase { surface_form: "amidy", expected_root: "varotra", category: "irregular_amidy_passive" },
    TestCase { surface_form: "namidy", expected_root: "varotra", category: "irregular_namidy_past_passive" },

    TestCase { surface_form: "misy", expected_root: "misy", category: "irregular_misy_present" },
    TestCase { surface_form: "nisy", expected_root: "misy", category: "irregular_nisy_past" },
    TestCase { surface_form: "fisiana", expected_root: "misy", category: "irregular_fisiana_circumstantial" },

    TestCase { surface_form: "mandre", expected_root: "re", category: "irregular_mandre" },
    TestCase { surface_form: "nandre", expected_root: "re", category: "irregular_nandre" },
    TestCase { surface_form: "renesina", expected_root: "re", category: "irregular_renesina_passive" },

    TestCase { surface_form: "mamoaka", expected_root: "voaka", category: "irregular_mamoaka" },
    TestCase { surface_form: "avoaka", expected_root: "voaka", category: "irregular_avoaka_passive" },

    TestCase { surface_form: "miditra", expected_root: "iditra", category: "irregular_miditra" },
    TestCase { surface_form: "ampidiro", expected_root: "iditra", category: "irregular_ampidiro_imperative" },
    TestCase { surface_form: "fidirana", expected_root: "iditra", category: "irregular_fidirana_circumstantial" },

    TestCase { surface_form: "miakatra", expected_root: "akatra", category: "irregular_miakatra" },
    TestCase { surface_form: "akarina", expected_root: "akatra", category: "irregular_akarina_passive" },

    TestCase { surface_form: "midina", expected_root: "dina", category: "irregular_midina" },
    TestCase { surface_form: "ampidinina", expected_root: "dina", category: "irregular_ampidinina_causative_passive" },
];

#[test]
fn test_golden_dataset_all_cases() {
    let mut passed = 0;
    let mut failed = 0;
    let mut failures = Vec::new();

    for case in GOLDEN_TESTSET {
        let actual = stem(case.surface_form);
        if actual == case.expected_root {
            passed += 1;
        } else {
            failed += 1;
            failures.push(format!(
                "FAIL [{}] input: '{}' -> expected '{}', got '{}'",
                case.category, case.surface_form, case.expected_root, actual
            ));
        }
    }

    if !failures.is_empty() {
        panic!(
            "Golden dataset test failed ({}/{} passed, {} failures):\n{}",
            passed,
            GOLDEN_TESTSET.len(),
            failed,
            failures.join("\n")
        );
    }

    assert_eq!(passed, GOLDEN_TESTSET.len());
}

#[test]
fn test_golden_dataset_full_paragraph_tokenization() {
    let text = "Nanoratra taratasy momba ny fampianarana sy ny fambolena ary ny fiarovana ny harem-pirenena tamin'ny solosaina ny mpianatra.";
    let roots = tokenize_and_stem(text, true);

    // Vérifier l'élimination des mots vides
    assert!(!roots.contains(&"ny".to_string()));
    assert!(!roots.contains(&"sy".to_string()));
    assert!(!roots.contains(&"ary".to_string()));
    assert!(!roots.contains(&"momba".to_string()));

    // Vérifier la présence des racines désaffixées
    assert!(roots.contains(&"soratra".to_string()), "manoratra/nanoratra -> soratra");
    assert!(roots.contains(&"taratasy".to_string()), "taratasy -> taratasy");
    assert!(roots.contains(&"anatra".to_string()), "fampianarana / mpianatra -> anatra");
    assert!(roots.contains(&"voly".to_string()), "fambolena -> voly");
    assert!(roots.contains(&"harena_firenena".to_string()), "harem-pirenena -> harena_firenena");
    assert!(roots.contains(&"solosaina".to_string()), "solosaina -> solosaina");
}
