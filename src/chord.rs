use itertools::join;

pub const KEYBOARD: &str = "\
\0─┬─┬┬─┬─┬─┬─┬┬─┬┬─┬─\0
\0 └┬┘└┬┘ │ └┬┘└┬┘└┬┘ \0
\0──┴──┴──┴──┴──┴──┴──\0";

#[derive(Debug, Clone)]
pub struct Chord<'a> {
    pub short_names: &'a [&'a str],
    // cdefgab and CDFGA ≡ c♯d♯f♯g♯a♯ and - as a seperator
    // \0─┬C┬┬D┬─┬─┬F┬┬G┬┬A┬─\0
    // \0 └┬┘└┬┘ │ └┬┘└┬┘└┬┘ \0
    // \0c─┴d─┴e─┴f─┴g─┴a─┴b─\0
    pub pattern: &'a str,
    pub names: &'a [&'a str],
}

impl<'a> Chord<'a> {
    pub const fn new(
        short_names: &'a [&'a str],
        pattern: &'a str,
        names: &'a [&'a str],
    ) -> Self {
        Self {
            short_names: short_names,
            pattern: pattern,
            names: names,
        }
    }

    pub fn both_names(&self) -> String {
        format!(
            "{} ({})",
            join(self.names, "|"),
            join(self.short_names, "|")
        )
    }

    pub fn keyboard(&self) -> String {
        let width: usize = KEYBOARD.chars().position(|c| c == '\n').expect("newline") + 1;
        let n_max: usize = self.pattern.chars().filter(|c| *c == '-').count();

        let mut board : Vec<String> = Vec::new();
        let mut n: usize = 0;

        let mut segment: Vec<char> = KEYBOARD.chars().collect();
        for ch in self.pattern.chars() {
            if ch == '-' {
                n += 1;
                segment.push('\n');
                board.push(segment.iter().collect::<String>());
                segment = KEYBOARD.chars().collect();
                continue;
            }

            if n == 0 {
                segment[0 * width] = if n == 0 { '┌' } else { '┬' };
                segment[1 * width] = '│';
                segment[2 * width] = if n == 0 { '└' } else { '┴' };
            }

            let idx: usize = match ch {
                // notes
                'C' => 0 * width + 3,
                'D' => 0 * width + 6,
                'F' => 0 * width + 12,
                'G' => 0 * width + 15,
                'A' => 0 * width + 18,
                'c' => 2 * width + 1,
                'd' => 2 * width + 4,
                'e' => 2 * width + 7,
                'f' => 2 * width + 10,
                'g' => 2 * width + 13,
                'a' => 2 * width + 16,
                'b' => 2 * width + 19,
                _ => panic!(),
            };
            segment[idx] = '●';

            segment[1 * width - 2] = if n == n_max { '┐' } else { '┬' };
            segment[2 * width - 2] = '│';
            segment[3 * width - 2] = if n == n_max { '┘' } else { '┴' };
        }
        segment.push('\n');
        board.push(segment.iter().collect::<String>());

        let lines: Vec<Vec<&str>> =
            board.iter().map(|s| s.lines().collect()).collect();

        (0..lines[0].len())
            .map(|i| {
                lines
                    .iter()
                    .map(|s| s[i])
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
