use itertools::join;

pub const KEYBOARD1: &str = "\
┌─┬─┬┬─┬─┬─┬─┬┬─┬┬─┬─┬─┬─┬┬─┬─┐
│ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │
└──┴──┴──┴──┴──┴──┴──┴──┴──┴──┘";

pub const KEYBOARD2: &str = "\
┌─┬─┬┬─┬┬─┬─┬─┬─┬┬─┬─┬─┬─┬┬─┬┬─┬─┐
│ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │
└──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┘";

#[derive(Debug, Clone)]
pub struct Chord<'a> {
    pub short_names: &'a [&'a str],
    // cdefgab and CDFGA ≡ c♯d♯f♯g♯a♯ AND
    // 01234 for the last notes (cc♯dd♯e) :
    // ┌─┬C┬┬D┬─┬─┬F┬┬G┬┬A┬─┬─┬1┬┬3┬─┐
    // │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │
    // └c─┴d─┴e─┴f─┴g─┴a─┴b─┴0─┴2─┴4─┘
    pub pattern1: &'a str,
    // fgabcde and FGACD ≡ f♯g♯a♯c♯d♯ AND
    // 0123456 for the last notes (ff♯gg♯ee♯d) :
    // ┌─┬F┬┬G┬┬A┬─┬─┬C┬┬D┬─┬─┬1┬┬3┬┬5┬─┐
    // │ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │
    // └f─┴g─┴a─┴b─┴c─┴d─┴e─┴0─┴2─┴4─┴6─┘
    pub pattern2: &'a str,
    pub names: &'a [&'a str],
}

impl<'a> Chord<'a> {
    pub const fn new(
        short_names: &'a [&'a str],
        pattern1: &'a str,
        pattern2: &'a str,
        names: &'a [&'a str],
    ) -> Self {
        Self {
            short_names: short_names,
            pattern1: pattern1,
            pattern2: pattern2,
            names: names,
        }
    }

    pub fn both_names(&self) -> String {
        format!("{} ({})",
                join(self.names, "|"),
                join(self.short_names, "|"))
    }

    pub fn keyboard1(&self) -> String {
        if self.pattern1.trim().is_empty() {
            return String::from("");
        }

        let mut board: Vec<char> = KEYBOARD1.chars().collect();
        let width: usize = board.iter().position(|c| *c == '\n').expect("newline") + 1;

        for ch in self.pattern1.chars() {
            let idx: usize = match ch {
                // notes
                'c' => 2 * width + 1,
                'd' => 2 * width + 4,
                'e' => 2 * width + 7,
                'f' => 2 * width + 10,
                'g' => 2 * width + 13,
                'a' => 2 * width + 16,
                'b' => 2 * width + 19,
                'C' => 3,
                'D' => 6,
                'F' => 12,
                'G' => 15,
                'A' => 18,
                // switch to position
                '0' => 2 * width + 22,
                '1' => 24,
                '2' => 2 * width + 25,
                '3' => 27,
                '4' => 2 * width + 28,
                _ => panic!(),
            };
            board[idx] = '●';
        }

        board.iter().collect()
    }

    pub fn keyboard2(&self) -> String {
        if self.pattern2.trim().is_empty() {
            return String::from("");
        }

        let mut board: Vec<char> = KEYBOARD2.chars().collect();
        let width: usize = board.iter().position(|c| *c == '\n').expect("newline") + 1;

        for ch in self.pattern2.chars() {
            let idx: usize = match ch {
                // notes
                'f' => 2 * width + 1,
                'g' => 2 * width + 4,
                'a' => 2 * width + 7,
                'b' => 2 * width + 10,
                'c' => 2 * width + 13,
                'd' => 2 * width + 16,
                'e' => 2 * width + 19,
                'F' => 3,
                'G' => 6,
                'A' => 9,
                'C' => 15,
                'D' => 18,
                // switch to position
                '0' => 2 * width + 22,
                '1' => 24,
                '2' => 2 * width + 25,
                '3' => 27,
                '4' => 2 * width + 28,
                '5' => 30,
                '6' => 2 * width + 31,
                _ => panic!(),
            };
            board[idx] = '●';
        }

        board.iter().collect()
    }

    pub fn keyboard(&self) -> String {
        let (kb1, kb2) = (self.keyboard1(), self.keyboard2());
        if kb1.is_empty() {
            kb2
        } else if kb2.is_empty() {
            kb1
        } else {  // interleave keyboard1 and keyboard2
            kb1.lines()
            .zip(kb2.lines())
            .map(|(a, b)| format!("{a}{b}"))
            .collect::<Vec<_>>()
            .join("\n")
        }
    }
}
