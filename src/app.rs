use wana_kana::ConvertJapanese;

use rand::Rng;

fn random_hiragana() -> String {
    let mut rng = rand::rng();

    String::from_utf16(&[rng.random_range(12353..12438)]).expect("error")
}

fn random_kana() -> String {
    random_hiragana()
}

pub fn get_hiragana() -> String {
    let coded_hira_vec: Vec<u16> = (12353..12438).collect();

    String::from_utf16(&coded_hira_vec).unwrap()
}

#[derive(Debug)]
pub struct App {
    pub shown: u16,
    pub correct: u16,
    pub input: String,
    pub current_kana: String,
    pub index: usize,
    pub display_answer: bool,
}

impl App {
    pub fn random(s: u16, c: u16) -> App {
        App {
            shown: s,
            correct: c,
            input: String::new(),
            current_kana: random_kana(),
            index: 0,
            display_answer: false,
        }
    }

    pub fn check_char(&mut self, c: char) {
        self.input.push(c);

        if self.input == self.current_kana.to_romaji() {
            if self.display_answer {
                self.display_answer = false;
            } else {
                self.correct += 1;
            }
            self.shown += 1;
            self.index = 0;
            self.input = String::new();
            self.current_kana = random_kana();
        }
    }

    pub fn reveal_roma(&mut self) {
        self.display_answer = true;
        self.input = String::new();
    }
}
