
static IRREGULAR_FORMS: &[(&str, &str)] = &[
    ("ahatongavana", "tonga"),
    ("akany", "aka"),
    ("akarina", "akatra"),
    ("akaro", "akatra"),
    ("alaina", "aka"),
    ("alao", "aka"),
    ("alaona", "aka"),
    ("amidio", "varotra"),
    ("amidy", "varotra"),
    ("ampidinina", "dina"),
    ("ampidino", "dina"),
    ("ampidirina", "iditra"),
    ("ampidiro", "iditra"),
    ("andeha", "leha"),
    ("andehanana", "leha"),
    ("andrenesana", "re"),
    ("anomezana", "ome"),
    ("atao", "tao"),
    ("ataovy", "tao"),
    ("avoahy", "voaka"),
    ("avoaka", "voaka"),
    ("entana", "entana"),
    ("entina", "tondra"),
    ("ento", "tondra"),
    ("fahalalana", "lala"),
    ("fahatonga", "tonga"),
    ("fahatongavana", "tonga"),
    ("famidiana", "varotra"),
    ("famoahana", "voaka"),
    ("famoaka", "voaka"),
    ("fanao", "tao"),
    ("fanaovana", "tao"),
    ("fandeha", "leha"),
    ("fandehanana", "leha"),
    ("fanisana", "isa"),
    ("fantarina", "fantatra"),
    ("fantaro", "fantatra"),
    ("fatoriana", "tory"),
    ("fiaviana", "avy"),
    ("fidinana", "dina"),
    ("fidirana", "iditra"),
    ("fifohazana", "foha"),
    ("fihainoana", "haino"),
    ("fihinana", "hano"),
    ("fihinanana", "hano"),
    ("fipetrahana", "petraka"),
    ("fisiana", "misy"),
    ("fisianana", "misy"),
    ("fitazanana", "tazana"),
    ("fitokisana", "toky"),
    ("fitondrana", "tondra"),
    ("fivaliana", "valy"),
    ("fivarotana", "varotra"),
    ("fohazina", "foha"),
    ("fohazo", "foha"),
    ("frenesana", "re"),
    ("frenesina", "re"),
    ("hahafantatra", "fantatra"),
    ("hahalala", "lala"),
    ("hakarina", "akatra"),
    ("hakeo", "aka"),
    ("halaina", "aka"),
    ("hamidy", "varotra"),
    ("hamoaka", "voaka"),
    ("hampidinina", "dina"),
    ("hampidirina", "iditra"),
    ("handeha", "leha"),
    ("handehana", "leha"),
    ("handehanana", "leha"),
    ("handositra", "lositra"),
    ("handre", "re"),
    ("handrenesana", "re"),
    ("hanina", "hano"),
    ("hanisa", "isa"),
    ("hanome", "ome"),
    ("hatao", "tao"),
    ("hatory", "tory"),
    ("havoaka", "voaka"),
    ("hentina", "tondra"),
    ("hiakatra", "akatra"),
    ("hiaviana", "avy"),
    ("hidina", "dina"),
    ("hidirana", "iditra"),
    ("hiditra", "iditra"),
    ("hifoha", "foha"),
    ("hihaino", "haino"),
    ("hihinana", "hano"),
    ("hihinanana", "hano"),
    ("hipetraka", "petraka"),
    ("hisiana", "misy"),
    ("hisianana", "misy"),
    ("hisy", "misy"),
    ("hitazana", "tazana"),
    ("hitondra", "tondra"),
    ("hivarotra", "varotra"),
    ("hividy", "vidy"),
    ("hofantarina", "fantatra"),
    ("hofohazina", "foha"),
    ("hohainoina", "haino"),
    ("hohanina", "hano"),
    ("holazaina", "laza"),
    ("holosirina", "lositra"),
    ("homana", "hano"),
    ("homena", "ome"),
    ("horenesina", "re"),
    ("hotoriana", "tory"),
    ("hovidina", "vidy"),
    ("iaviana", "avy"),
    ("idinana", "dina"),
    ("idirana", "iditra"),
    ("ihinanana", "hano"),
    ("isianana", "misy"),
    ("kohanana", "hano"),
    ("kohanina", "hano"),
    ("losirina", "lositra"),
    ("losiro", "lositra"),
    ("mahafantatra", "fantatra"),
    ("mahalala", "lala"),
    ("maka", "aka"),
    ("mamoaka", "voaka"),
    ("mampiditra", "iditra"),
    ("mandeha", "leha"),
    ("mandehana", "leha"),
    ("mandositra", "lositra"),
    ("mandre", "re"),
    ("manisa", "isa"),
    ("manome", "ome"),
    ("matory", "tory"),
    ("miakatra", "akatra"),
    ("midina", "dina"),
    ("miditra", "iditra"),
    ("mifoha", "foha"),
    ("mihaino", "haino"),
    ("mihinana", "hano"),
    ("mipetraka", "petraka"),
    ("misy", "misy"),
    ("mitazana", "tazana"),
    ("mitondra", "tondra"),
    ("mivarotra", "varotra"),
    ("mividy", "vidy"),
    ("mpahalala", "lala"),
    ("mpahatonga", "tonga"),
    ("mpaka", "aka"),
    ("mpamoaka", "voaka"),
    ("mpampiditra", "iditra"),
    ("mpandeha", "leha"),
    ("mpandositra", "lositra"),
    ("mpanisa", "isa"),
    ("mpanome", "ome"),
    ("mpatory", "tory"),
    ("mpiakatra", "akatra"),
    ("mpidina", "dina"),
    ("mpiditra", "iditra"),
    ("mpifoha", "foha"),
    ("mpihaino", "haino"),
    ("mpihinana", "hano"),
    ("mpitondra", "tondra"),
    ("nahafantatra", "fantatra"),
    ("nahalala", "lala"),
    ("nahatonga", "tonga"),
    ("naka", "aka"),
    ("nakarina", "akatra"),
    ("nalaina", "aka"),
    ("nalao", "aka"),
    ("namidy", "varotra"),
    ("namoaka", "voaka"),
    ("nampidinina", "dina"),
    ("nampidirina", "iditra"),
    ("nandeha", "leha"),
    ("nandehanana", "leha"),
    ("nandositra", "lositra"),
    ("nandre", "re"),
    ("nandrenesana", "re"),
    ("nanisa", "isa"),
    ("nanome", "ome"),
    ("natao", "tao"),
    ("natory", "tory"),
    ("navoaka", "voaka"),
    ("nentina", "tondra"),
    ("niakatra", "akatra"),
    ("niaviana", "avy"),
    ("nidina", "dina"),
    ("nidirana", "iditra"),
    ("niditra", "iditra"),
    ("nifoha", "foha"),
    ("nihaino", "haino"),
    ("nihinana", "hano"),
    ("nihinanana", "hano"),
    ("nipetraka", "petraka"),
    ("nisianana", "misy"),
    ("nisy", "misy"),
    ("nitazana", "tazana"),
    ("nitondra", "tondra"),
    ("nivarotra", "varotra"),
    ("nividy", "vidy"),
    ("nofantarina", "fantatra"),
    ("nofohazina", "foha"),
    ("nohainoina", "haino"),
    ("nohanina", "hano"),
    ("nolazaina", "laza"),
    ("nolosirina", "lositra"),
    ("nomena", "ome"),
    ("norenesina", "re"),
    ("notenenina", "teny"),
    ("notoriana", "tory"),
    ("novaliana", "valy"),
    ("novidina", "vidy"),
    ("omena", "ome"),
    ("omeo", "ome"),
    ("petrahana", "petraka"),
    ("renesina", "re"),
    ("tazanina", "tazana"),
    ("tenenina", "teny"),
    ("teneno", "teny"),
    ("tokisana", "toky"),
    ("toriana", "tory"),
    ("torio", "tory"),
    ("valiana", "valy"),
    ("vidina", "vidy"),
    ("vidio", "vidy"),
];

pub fn lookup_irregular(word: &str) -> Option<&'static str> {
    IRREGULAR_FORMS
        .binary_search_by_key(&word, |&(surface, _)| surface)
        .ok()
        .map(|idx| IRREGULAR_FORMS[idx].1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irregular_forms_sorted() {
        for window in IRREGULAR_FORMS.windows(2) {
            assert!(
                window[0].0 < window[1].0,
                "Table IRREGULAR_FORMS non triée : '{}' >= '{}'",
                window[0].0,
                window[1].0
            );
        }
    }

    #[test]
    fn test_lookup_mandeha_series() {
        assert_eq!(lookup_irregular("mandeha"), Some("leha"));
        assert_eq!(lookup_irregular("nandeha"), Some("leha"));
        assert_eq!(lookup_irregular("handeha"), Some("leha"));
        assert_eq!(lookup_irregular("fandehanana"), Some("leha"));
        assert_eq!(lookup_irregular("mandehana"), Some("leha"));
    }

    #[test]
    fn test_lookup_homana_series() {
        assert_eq!(lookup_irregular("homana"), Some("hano"));
        assert_eq!(lookup_irregular("mihinana"), Some("hano"));
        assert_eq!(lookup_irregular("hanina"), Some("hano"));
        assert_eq!(lookup_irregular("nohanina"), Some("hano"));
        assert_eq!(lookup_irregular("fihinanana"), Some("hano"));
    }

    #[test]
    fn test_lookup_transport_entina() {
        assert_eq!(lookup_irregular("entina"), Some("tondra"));
        assert_eq!(lookup_irregular("nentina"), Some("tondra"));
        assert_eq!(lookup_irregular("ento"), Some("tondra"));
    }

    #[test]
    fn test_lookup_taking_alaina() {
        assert_eq!(lookup_irregular("alaina"), Some("aka"));
        assert_eq!(lookup_irregular("nalaina"), Some("aka"));
        assert_eq!(lookup_irregular("maka"), Some("aka"));
    }

    #[test]
    fn test_lookup_giving_manome() {
        assert_eq!(lookup_irregular("manome"), Some("ome"));
        assert_eq!(lookup_irregular("omena"), Some("ome"));
        assert_eq!(lookup_irregular("omeo"), Some("ome"));
    }

    #[test]
    fn test_lookup_selling_amidy() {
        assert_eq!(lookup_irregular("amidy"), Some("varotra"));
        assert_eq!(lookup_irregular("namidy"), Some("varotra"));
        assert_eq!(lookup_irregular("hamidy"), Some("varotra"));
    }

    #[test]
    fn test_lookup_non_irregular() {
        assert_eq!(lookup_irregular("manoratra"), None);
        assert_eq!(lookup_irregular("boky"), None);
    }
}
