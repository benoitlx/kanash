use wana_kana::ConvertJapanese;

use rand::Rng;

fn get_hiragana() -> String {
    let mut rng = rand::rng();

    String::from_utf16(&[rng.random_range(12353..12438)]).expect("error")
}

fn get_kana() -> String {
    get_hiragana()
}

#[derive(Debug)]
pub struct App {
    pub shown: u16,
    pub correct: u16,
    pub input: String,
    pub current_kana: String,
    pub index: usize,
}

impl App {
    pub fn random(s: u16, c: u16) -> App {
        App {
            shown: s,
            correct: c,
            input: String::new(),
            current_kana: get_kana(),
            index: 0,
        }
    }

    pub fn check_char(&mut self, c: char) {
        self.input.push(c);

        if self.input == self.current_kana.to_romaji() {
            self.correct += 1;
            self.shown += 1;
            self.index = 0;
            self.input = String::new();
            self.current_kana = get_kana();
        }
    }

    pub fn reveal_roma(&mut self) {
        todo!()
    }
}
