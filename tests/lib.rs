#[macro_use]
extern crate lazy_static;
extern crate quickcheck;

use std::path::PathBuf;
use quickcheck::{quickcheck};

extern crate hyphenation;
use hyphenation::{load, Language, Corpus, Hyphenation, Standard};


fn fiat_io(lang: Language) -> Corpus {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("patterns");

    hyphenation::set_pattern_folder(path.as_path());
    load::language(lang).unwrap()
}

lazy_static! {
    static ref EN_US: Corpus = fiat_io(Language::English_US);
}


#[test]
fn collected_equals_original() {
    fn property(original: String) -> bool {
        let collected: String = original.hyphenate(&EN_US).collect();

        collected == original
    }

    quickcheck(property as fn(String) -> bool);
}

#[test]
fn opportunities_within_bounds() {
    fn property(s: String) -> bool {
        let os = s.opportunities(&EN_US);
        let l = s.len();

        os.iter().all(|&i| i < l)
    }

    quickcheck(property as fn(String) -> bool);
}

#[test]
fn punctuated_count() {
    fn property(s: String) -> bool {
        let l = s.chars().count();
        let os = s.opportunities(&EN_US);
        let h: String = s.hyphenate(&EN_US).punctuate().collect();

        h.chars().count() == l + os.len()
    }

    quickcheck(property as fn(String) -> bool);
}

#[test]
fn basics() {
    let h1: Standard = "hyphenation".hyphenate(&EN_US);
    let h2: Standard = "project".hyphenate(&EN_US);
    let h3: Standard = "hypha".hyphenate(&EN_US);

    let v1: Vec<&str> = h1.clone().collect();
    let v2: Vec<&str> = h2.clone().collect();
    let v3: Vec<&str> = h3.clone().collect();
    assert_eq!(v1, vec!["hy", "phen", "ation"]);
    assert_eq!(v2, vec!["project"]);
    assert_eq!(v3, vec!["hy", "pha"]);

    let s1: String = h1.punctuate().collect();
    assert_eq!(s1, "hy\u{ad}phen\u{ad}ation".to_owned());
}
