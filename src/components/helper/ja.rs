use rand::Rng;

fn random_hiragana() -> String {
    let mut rng = rand::rng();

    String::from_utf16(&[rng.random_range(12353..12438)]).expect("error")
}

pub fn random_kana() -> String {
    random_hiragana()
}

pub fn get_hiragana() -> String {
    let coded_hira_vec: Vec<u16> = (12353..12438).collect();

    String::from_utf16(&coded_hira_vec).unwrap()
}