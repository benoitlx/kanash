pub const HIRAGANA_NUMBER: usize = 71;
pub const WANTED_HIRAGANA: [u16; HIRAGANA_NUMBER] = [
    12354, 12356, 12358, 12360, 12362, 12363, 12364, 12365, 12366, 12367, 12368, 12369, 12370,
    12371, 12372, 12373, 12374, 12375, 12376, 12377, 12378, 12379, 12380, 12381, 12382, 12383,
    12384, 12385, 12386, 12388, 12389, 12390, 12391, 12392, 12393, 12394, 12395, 12396, 12397,
    12398, 12399, 12400, 12401, 12402, 12403, 12404, 12405, 12406, 12407, 12408, 12409, 12410,
    12411, 12412, 12413, 12414, 12415, 12416, 12417, 12418, 12420, 12422, 12424, 12425, 12426,
    12427, 12428, 12429, 12431, 12434, 12435,
];
pub const KATAKANA_NUMBER: usize = 81;
pub const WANTED_KATAKANA: [u16; KATAKANA_NUMBER] = [
    12449, 12450, 12451, 12452, 12453, 12454, 12455, 12456, 12457, 12458, 12459, 12460, 12461,
    12462, 12463, 12464, 12465, 12466, 12467, 12468, 12469, 12470, 12471, 12472, 12473, 12474,
    12475, 12476, 12477, 12478, 12479, 12480, 12481, 12482, 12484, 12485, 12486, 12487, 12488,
    12489, 12490, 12491, 12492, 12493, 12494, 12495, 12496, 12497, 12498, 12499, 12500, 12501,
    12502, 12503, 12504, 12505, 12506, 12507, 12508, 12509, 12510, 12511, 12512, 12513, 12514,
    12515, 12516, 12517, 12518, 12519, 12520, 12521, 12522, 12523, 12524, 12525, 12526, 12527,
    12530, 12531, 12532,
];

pub fn random_hiragana() -> (String, usize) {
    let index = rand::random_range(0..HIRAGANA_NUMBER);
    (
        String::from_utf16(&[WANTED_HIRAGANA[index]]).expect("error hiragana"),
        index,
    )
}

pub fn random_katakana() -> (String, usize) {
    let index = rand::random_range(0..KATAKANA_NUMBER);
    (
        String::from_utf16(&[WANTED_KATAKANA[index]]).expect("error katakana"),
        index + HIRAGANA_NUMBER,
    )
}

pub fn random_kana() -> (String, usize) {
    if rand::random() {
        random_hiragana()
    } else {
        random_katakana()
    }
}
