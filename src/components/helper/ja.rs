use rand::{Rng, SeedableRng};
use rand_pcg::{Mcg128Xsl64, Pcg64Mcg};

fn random_hiragana(rng: &mut Mcg128Xsl64) -> String {
    String::from_utf16(&[rng.random_range(12353..12438)]).expect("error")
}

pub fn random_kana(rng: &mut Mcg128Xsl64) -> String {
    random_hiragana(rng)
}

pub fn get_hiragana() -> String {
    let coded_hira_vec: Vec<u16> = (12353..12438).collect();

    String::from_utf16(&coded_hira_vec).unwrap()
}
