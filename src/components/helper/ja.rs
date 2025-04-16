use rand::{seq::IndexedRandom, Rng, SeedableRng};
use rand_pcg::{Mcg128Xsl64, Pcg64Mcg};

const KANA_NUMBER: usize = 71;
const WANTED_KANA: [u16; KANA_NUMBER] = [
    12354, 12356, 12358, 12360, 12362, 12363, 12364, 12365, 12366, 12367, 12368, 12369, 12370,
    12371, 12372, 12373, 12374, 12375, 12376, 12377, 12378, 12379, 12380, 12381, 12382, 12383,
    12384, 12385, 12386, 12388, 12389, 12390, 12391, 12392, 12393, 12394, 12395, 12396, 12397,
    12398, 12399, 12400, 12401, 12402, 12403, 12404, 12405, 12406, 12407, 12408, 12409, 12410,
    12411, 12412, 12413, 12414, 12415, 12416, 12417, 12418, 12420, 12422, 12424, 12425, 12426,
    12427, 12428, 12429, 12431, 12434, 12435,
];

fn random_hiragana(rng: &mut Mcg128Xsl64) -> String {
    String::from_utf16(&[*WANTED_KANA.choose(rng).unwrap()]).expect("error")
}

pub fn random_kana(rng: &mut Mcg128Xsl64) -> String {
    random_hiragana(rng)
}

pub fn get_hiragana() -> String {
    String::from_utf16(&WANTED_KANA).unwrap()
}
