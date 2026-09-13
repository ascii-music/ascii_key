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
    Chord::new(&["C#m7", "Dbm7"], "CeGb", "", &["C♯ minor 7ᵗʰ", "D♭ minor 7ᵗʰ"]),
    Chord::new(&["C#maj7", "Dbmaj7"], "CfG0", "", &["C♯ raised 7ᵗʰ", "D♭ raised 7ᵗʰ"]),
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
    Chord::new(&["D#m7", "Ebm7"], "DFA1", "", &["D♯ minor 7ᵗʰ", "E♭ minor 7ᵗʰ"]),
    Chord::new(&["D#maj7", "Ebmaj7"], "DgA2", "", &["D♯ raised 7ᵗʰ", "E♭ raised 7ᵗʰ"]),
    Chord::new(&["E"], "eGb", "", &["E"]),
    Chord::new(&["E7"], "eGb2", "", &["E 7ᵗʰ"]),
    Chord::new(&["Emaj7"], "eGb3", "", &["E raised 7ᵗʰ"]),
    Chord::new(&["Esus"], "eab", "", &["E suspended"]),
    Chord::new(&["E6"], "eGb1", "", &["E 6ᵗʰ"]),
    Chord::new(&["Eadd2"], "eFGb", "", &["E added 2ⁿᵈ"]),
    Chord::new(&["Em"], "egb", "", &["E minor"]),
    Chord::new(&["F"], "fa0", "", &["F"]),
    Chord::new(&["Fm"], "fG0", "", &["F minor"]),
    // A/C# is an A major chord with C# as the bass note and A/E is an A major chord with E as the bass note
    Chord::new(&["A/C#", "A/Db"], "Cea", "", &["A over C♯", "A over D♭"]),
    Chord::new(&["A/E"], "ea1", "", &["A over E"]),
    // keyboard 2 ≡ upper part
    // ┌─┬F┬┬G┬┬A┬─┬─┬C┬┬D┬─┬─┬1┬┬3┬┬5┬─┐
    // │ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │
    // └f─┴g─┴a─┴b─┴c─┴d─┴e─┴0─┴2─┴4─┴6─┘
    Chord::new(&["F#", "Gb"], "", "FAC", &["F♯", "G♭"]),
    Chord::new(&["F#7", "Gb7"], "", "FACe", &["F♯ 7ᵗʰ", "G♭ 7ᵗʰ"]),
    Chord::new(&["F#m", "Gbm"], "", "FaC", &["F♯ minor", "G♭ minor"]),
    Chord::new(&["F#m7", "Gbm7"], "", "FaCe", &["F♯ minor 7ᵗʰ", "G♭ minor 7ᵗʰ"]),
    Chord::new(&["F#maj7", "Gbmaj7"], "", "FAC0", &["F♯ raised 7ᵗʰ", "G♭ raised 7ᵗʰ"]),
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
    Chord::new(&["G#m"], "", "GbD", &["G♯ minor", "A♭ minor"]),
    Chord::new(&["G#m7", "Abm7"], "", "GbD1", &["G♯ minor 7ᵗʰ", "A♭ minor 7ᵗʰ"]),
    Chord::new(&["G#maj7", "Abmaj7"], "GcD2", "", &["G♯ raised 7ᵗʰ", "A♭ raised 7ᵗʰ"]),
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
    Chord::new(&["A#m7", "Bbm7"], "", "AC03", &["A♯ minor 7ᵗʰ", "B♭ minor 7ᵗʰ"]),
    Chord::new(&["A#maj7", "Bbmaj7"], "", "Ad04", &["A♯ raised 7ᵗʰ", "B♭ raised 7ᵗʰ"]),
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

    // sources
    // pianowithjonny.com/piano-lessons/major-7th-chords-for-piano-a-complete-guide
];

pub static ALL_CHORDS_BY_SHORT_NAMES: Lazy<HashMap<String, Vec<&'static Chord<'static>>>> =
    Lazy::new(|| {
        let mut map = HashMap::<_, Vec<_>>::new();

        for chord in ALL_CHORDS {
            for sn in chord.short_names {
                map.entry(sn.to_ascii_lowercase())
                    .or_default()
                    .push(chord);
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
        for chord in ALL_CHORDS {
            assert!(chord.pattern1.is_empty() || chord.pattern2.is_empty())
        }
    }
}
