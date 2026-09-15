use crate::chord::Chord;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ALL_CHORDS: &'static [Chord] = &[
    // NOTE: major is omitted such that A ≡ A major by default
    // keyboard 1 ≡ lower part
    // ┌─┬C┬┬D┬─┬─┬F┬┬G┬┬A┬─┬─┬1┬┬3┬─┐
    // │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │
    // └c─┴d─┴e─┴f─┴g─┴a─┴b─┴0─┴2─┴4─┘
    Chord::new(&["C"], "ceg", "", &["C"]),
    Chord::new(&["C7"], "cegA", "", &["C 7ᵗʰ"]),
    Chord::new(&["Cmaj7"], "cegb", "", &["C raised 7ᵗʰ"]),
    Chord::new(&["Csus"], "cfg", "", &["C suspended"]),
    Chord::new(&["C6"], "cega", "", &["C 6ᵗʰ"]),
    Chord::new(&["Cadd2"], "cdeg", "", &["C added 2ⁿᵈ"]),
    Chord::new(&["Cm"], "cDg", "", &["C minor"]),
    Chord::new(&["Cm7"], "cDgA", "", &["C minor 7ᵗʰ"]),
    // Chord::new(&["Cm7b5"], "", "", &[""]),
    Chord::new(&["Cdim"], "cDF", "", &["C diminished"]),
    Chord::new(&["Cdim7"], "cDFa", "", &["C diminished 7ᵗʰ"]),
    Chord::new(&["C+"], "ceG", "", &["C augmented"]),
    Chord::new(&["C#", "Db"], "CfG", "", &["C♯ ", "D♭"]),
    Chord::new(&["C#7", "Db7"], "CfGb", "", &["C♯ 7ᵗʰ", "D♭ 7ᵗʰ"]),
    Chord::new(&["C#m", "Dbm"], "CeG", "", &["C♯ minor", "D♭ minor"]),
    Chord::new(
        &["C#m7", "Dbm7"],
        "CeGb",
        "",
        &["C♯ minor 7ᵗʰ", "D♭ minor 7ᵗʰ"],
    ),
    Chord::new(
        &["C#maj7", "Dbmaj7"],
        "CfG0",
        "",
        &["C♯ raised 7ᵗʰ", "D♭ raised 7ᵗʰ"],
    ),
    Chord::new(&["D"], "dFa", "", &["D"]),
    Chord::new(&["D7"], "dFa0", "", &["D 7ᵗʰ"]),
    Chord::new(&["Dmaj7"], "dFa1", "", &["D raised 7ᵗʰ"]),
    Chord::new(&["Dsus"], "dga", "", &["D suspended"]),
    Chord::new(&["D6"], "dFga", "", &["D 6ᵗʰ"]),
    Chord::new(&["Dadd2"], "deFa", "", &["D added 2ⁿᵈ"]),
    Chord::new(&["Dm"], "dfa", "", &["D minor"]),
    Chord::new(&["Dm7"], "dfa0", "", &["D minor 7ᵗʰ"]),
    Chord::new(&["D#", "Eb"], "DgA", "", &["D♯", "E♭"]),
    Chord::new(&["D#7", "Eb7"], "DgA1", "", &["D♯ 7ᵗʰ", "E♭ 7ᵗʰ"]),
    Chord::new(&["D#m", "Ebm"], "DFA", "", &["D♯ minor", "E♭ minor"]),
    Chord::new(
        &["D#m7", "Ebm7"],
        "DFA1",
        "",
        &["D♯ minor 7ᵗʰ", "E♭ minor 7ᵗʰ"],
    ),
    Chord::new(
        &["D#maj7", "Ebmaj7"],
        "DgA2",
        "",
        &["D♯ raised 7ᵗʰ", "E♭ raised 7ᵗʰ"],
    ),
    Chord::new(&["E"], "eGb", "", &["E"]),
    Chord::new(&["E7"], "eGb2", "", &["E 7ᵗʰ"]),
    Chord::new(&["Emaj7"], "eGb3", "", &["E raised 7ᵗʰ"]),
    Chord::new(&["Esus"], "eab", "", &["E suspended"]),
    Chord::new(&["E6"], "eGb1", "", &["E 6ᵗʰ"]),
    Chord::new(&["Eadd2"], "eFGb", "", &["E added 2ⁿᵈ"]),
    Chord::new(&["Em"], "egb", "", &["E minor"]),
    Chord::new(&["F"], "fa0", "", &["F"]),
    Chord::new(&["F7"], "fa03", "", &["F 7ᵗʰ"]),
    Chord::new(&["Fm"], "fG0", "", &["F minor"]),
    // keyboard 2 ≡ upper part
    // ┌─┬F┬┬G┬┬A┬─┬─┬C┬┬D┬─┬─┬1┬┬3┬┬5┬─┐
    // │ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │
    // └f─┴g─┴a─┴b─┴c─┴d─┴e─┴0─┴2─┴4─┴6─┘
    Chord::new(&["F#", "Gb"], "", "FAC", &["F♯", "G♭"]),
    Chord::new(&["F#7", "Gb7"], "", "FACe", &["F♯ 7ᵗʰ", "G♭ 7ᵗʰ"]),
    Chord::new(&["F#m", "Gbm"], "", "FaC", &["F♯ minor", "G♭ minor"]),
    Chord::new(
        &["F#m7", "Gbm7"],
        "",
        "FaCe",
        &["F♯ minor 7ᵗʰ", "G♭ minor 7ᵗʰ"],
    ),
    Chord::new(
        &["F#maj7", "Gbmaj7"],
        "",
        "FAC0",
        &["F♯ raised 7ᵗʰ", "G♭ raised 7ᵗʰ"],
    ),
    Chord::new(&["G"], "", "gbd", &["G"]),
    Chord::new(&["G7"], "", "gbd0", &["G 7ᵗʰ"]),
    Chord::new(&["Gmaj7"], "", "gbd1", &["G raised 7ᵗʰ"]),
    Chord::new(&["Gsus"], "", "gcd", &["G suspended"]),
    Chord::new(&["G6"], "", "gbde", &["G 6ᵗʰ"]),
    Chord::new(&["Gadd2"], "", "gabd", &["G added 2ⁿᵈ"]),
    Chord::new(&["Gm"], "", "gAd", &["G minor"]),
    Chord::new(&["Gm7"], "", "gAd0", &["G minor 7ᵗʰ"]),
    // Chord::new(&["Gm7b5"], "", "", &[""]),
    Chord::new(&["Gdim"], "", "gAC", &["G diminished"]),
    Chord::new(&["Gdim7"], "", "gACe", &["G diminished 7ᵗʰ"]),
    Chord::new(&["G+"], "", "gbD", &["G augmented"]),
    Chord::new(&["G#", "Ab"], "", "GcD", &["G♯", "A♭"]),
    Chord::new(&["G#7", "Ab7"], "", "GcD1", &["G♯ 7ᵗʰ", "A♭ 7ᵗʰ"]),
    Chord::new(&["G#m", "Abm"], "", "GbD", &["G♯ minor", "A♭ minor"]),
    Chord::new(
        &["G#m7", "Abm7"],
        "",
        "GbD1",
        &["G♯ minor 7ᵗʰ", "A♭ minor 7ᵗʰ"],
    ),
    Chord::new(
        &["G#maj7", "Abmaj7"],
        "GcD2",
        "",
        &["G♯ raised 7ᵗʰ", "A♭ raised 7ᵗʰ"],
    ),
    Chord::new(&["A"], "", "aCe", &["A"]),
    Chord::new(&["A7"], "", "aCe2", &["A 7ᵗʰ"]),
    Chord::new(&["Amaj7"], "", "aCe3", &["A raised 7ᵗʰ"]),
    Chord::new(&["Asus"], "", "ade", &["A suspended"]),
    Chord::new(&["A6"], "", "aCe1", &["A 6ᵗʰ"]),
    Chord::new(&["Aadd2"], "", "abCe", &["A added 2ⁿᵈ"]),
    Chord::new(&["Am"], "", "ace", &["A minor"]),
    Chord::new(&["Am7"], "", "ace2", &["A minor 7ᵗʰ"]),
    // Chord::new(&["Am7b5"], "", "", &[""]),
    Chord::new(&["Adim"], "", "acD", &["A diminished"]),
    Chord::new(&["Adim7"], "", "acD1", &["A diminished 7ᵗʰ"]),
    Chord::new(&["A+"], "", "aC0", &["A augmented"]),
    Chord::new(&["A#", "Bb"], "", "Ad0", &["A♯", "B♭"]),
    Chord::new(&["A#7", "Bb7"], "", "Ad03", &["A♯ 7ᵗʰ", "B♭ 7ᵗʰ"]),
    Chord::new(&["A#m", "Bbm"], "", "AC0", &["A♯ minor", "B♭ minor"]),
    Chord::new(
        &["A#m7", "Bbm7"],
        "",
        "AC03",
        &["A♯ minor 7ᵗʰ", "B♭ minor 7ᵗʰ"],
    ),
    Chord::new(
        &["A#maj7", "Bbmaj7"],
        "",
        "Ad04",
        &["A♯ raised 7ᵗʰ", "B♭ raised 7ᵗʰ"],
    ),
    Chord::new(&["B"], "", "bD1", &["B"]),
    Chord::new(&["B7"], "", "bD14", &["B 7ᵗʰ"]),
    Chord::new(&["Bmaj7"], "", "bD15", &["B raised 7ᵗʰ"]),
    Chord::new(&["Bsus"], "", "be1", &["B suspended"]),
    Chord::new(&["B6"], "", "bD13", &["B 6ᵗʰ"]),
    Chord::new(&["Badd2"], "", "bCD1", &["B added 2ⁿᵈ"]),
    Chord::new(&["Bm"], "", "bd1", &["B minor"]),
    Chord::new(&["Bm7"], "", "bd14", &["B minor 7ᵗʰ"]),
    // Chord::new(&["Bm7b5"], "", "", &[""]),
    Chord::new(&["Bdim"], "", "bd0", &["B diminished"]),
    Chord::new(&["Bdim7"], "", "bd03", &["B diminished 7ᵗʰ"]),
    Chord::new(&["B+"], "", "bD2", &["B augmented"]),
    // slash / inversion chords (some might be hybrid)
    // ┌─┬C┬┬D┬─┬─┬F┬┬G┬┬A┬─┬─┬1┬┬3┬─┐┌─┬F┬┬G┬┬A┬─┬─┬C┬┬D┬─┬─┬1┬┬3┬┬5┬─┐
    // │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ ││ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │
    // └c─┴d─┴e─┴f─┴g─┴a─┴b─┴0─┴2─┴4─┘└f─┴g─┴a─┴b─┴c─┴d─┴e─┴0─┴2─┴4─┴6─┘
    // A/C# is an A major chord with C# as the bass note and A/E is an A major chord with E as the bass note
    // pianochord.org/c-major.html#hide1
    Chord::new(&["C/E"], "eg0", "", &["C over E"]),
    Chord::new(&["C/G"], "g04", "", &["C over G"]),
    // pianochord.org/d-major.html#hide1
    Chord::new(&["D/F#", "D/Gb"], "", "Fad", &["D over F♯", "D over G♭"]),
    Chord::new(&["D/A"], "", "ad1", &["D over A"]),
    // pianochord.org/e-major.html#hide1
    Chord::new(&["E/G#", "E/Ab"], "", "Gbe", &["E over G♯", "E over A♭"]),
    Chord::new(&["E/B"], "", "be3", &["E over B"]),
    // pianochord.org/f-major.html#hide1
    Chord::new(&["F/A"], "", "ac0", &["F over A"]),
    Chord::new(&["F/C"], "", "c04", &["F over C"]),
    // pianochord.org/g-major.html#hide1
    Chord::new(&["G/B"], "", "bd2", &["G over B"]),
    Chord::new(&["G/D"], "", "d26", &["G over B"]),
    // pianochord.org/a-major.html#hide1
    Chord::new(&["A/C#", "A/Db"], "Cea", "", &["A over C♯", "A over D♭"]),
    Chord::new(&["A/E"], "ea1", "", &["A over E"]),
    // pianochord.org/b-major.html#hide1
    Chord::new(&["B/D#", "B/Eb"], "DFb", "", &["B over D♯", "B over E♭"]),
    Chord::new(&["B/F#", "B/Gb"], "Fb3", "", &["B over F♯", "B over G♭"]),
    // sources
    // pianowithjonny.com/piano-lessons/major-7th-chords-for-piano-a-complete-guide
    // pianowithjonny.com/piano-lessons/7th-chords-for-piano-the-complete-guide*
    // slash chords / inversions : piano-lessons-info.com/fmajorchord.html
];

pub static ALL_CHORDS_BY_SHORT_NAMES: Lazy<HashMap<String, Vec<&'static Chord<'static>>>> =
    Lazy::new(|| {
        let mut map = HashMap::<_, Vec<_>>::new();

        for chord in ALL_CHORDS {
            for sn in chord.short_names {
                map.entry(sn.to_ascii_lowercase()).or_default().push(chord);
            }
        }
        map
    });

#[cfg(test)]
mod tests {
    // NOTE: useful idiom - importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_single_pattern() {
        'outer: for chord in ALL_CHORDS {
            for short_name in chord.short_names {
                if short_name.contains('/') {
                    // slash chords might be hybrid
                    continue 'outer;
                }
            }
            assert!(chord.pattern1.is_empty() || chord.pattern2.is_empty())
        }
    }

    #[test]
    fn test_digit_or_char() {
        for chord in ALL_CHORDS {
            for chr in chord.pattern1.chars() {
                assert!("abcdefgACDEFG01234".contains(chr));
            }
            for chr in chord.pattern2.chars() {
                assert!("abcdefgACDEFG0123456".contains(chr));
            }
        }
    }

    #[test]
    fn test_aliases() {
        for chord in ALL_CHORDS {
            assert_eq!(
                chord.short_names.len(),
                chord.names.len(),
                "Aliases issue short_names={:?} names={:?}",
                chord.short_names,
                chord.names
            )
        }
    }
}
