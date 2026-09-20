use crate::chord::Chord;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[rustfmt::skip]
pub static ALL_CHORDS: &'static [Chord] = &[
    // NOTE: major is omitted such that A ≡ A major by default
    // ─┬C┬┬D┬─┬─┬F┬┬G┬┬A┬─
    //  └┬┘└┬┘ │ └┬┘└┬┘└┬┘ 
    // c─┴d─┴e─┴f─┴g─┴a─┴b─
    // use the `-` separator to add another section to the keyboard
    Chord::new(&["C"], "ceg", &["C"]),
    Chord::new(&["C7"], "cegA", &["C 7ᵗʰ"]),
    Chord::new(&["Cmaj7"], "cegb", &["C raised 7ᵗʰ"]),
    Chord::new(&["Csus"], "cfg", &["C suspended"]),
    Chord::new(&["C6"], "cega", &["C 6ᵗʰ"]),
    Chord::new(&["Cadd2"], "cdeg", &["C added 2ⁿᵈ"]),
    Chord::new(&["Cm"], "cDg", &["C minor"]),
    Chord::new(&["Cm7"], "cDgA", &["C minor 7ᵗʰ"]),
    Chord::new(&["Cm7b5"], "cDFA", &["C half-diminished"]),
    Chord::new(&["Cdim"], "cDF", &["C diminished"]),
    Chord::new(&["Cdim7"], "cDFa", &["C diminished 7ᵗʰ"]),
    Chord::new(&["C+"], "ceG", &["C augmented"]),
    Chord::new(&["C#", "Db"], "CfG", &["C♯ ", "D♭"]),
    Chord::new(&["C#7", "Db7"], "CfGb", &["C♯ 7ᵗʰ", "D♭ 7ᵗʰ"]),
    Chord::new(&["C#m", "Dbm"], "CeG", &["C♯ minor", "D♭ minor"]),
    Chord::new(&["C#m7", "Dbm7"], "CeGb", &["C♯ minor 7ᵗʰ", "D♭ minor 7ᵗʰ"]),
    Chord::new(&["C#maj7", "Dbmaj7"], "CfG-c", &["C♯ raised 7ᵗʰ", "D♭ raised 7ᵗʰ"]),
    Chord::new(&["D"], "dFa", &["D"]),
    Chord::new(&["D7"], "dFa-c", &["D 7ᵗʰ"]),
    Chord::new(&["Dmaj7"], "dFa-C", &["D raised 7ᵗʰ"]),
    Chord::new(&["Dsus"], "dga", &["D suspended"]),
    Chord::new(&["D6"], "dFAB", &["D 6ᵗʰ"]),
    Chord::new(&["Dadd2"], "deFa", &["D added 2ⁿᵈ"]),
    Chord::new(&["Dm"], "dfa", &["D minor"]),
    Chord::new(&["Dm7"], "dfa-c", &["D minor 7ᵗʰ"]),
    Chord::new(&["Dm7b5"], "dfG-c", &["D half-diminished"]),
    Chord::new(&["D#", "Eb"], "DgA", &["D♯", "E♭"]),
    Chord::new(&["D#7", "Eb7"], "DgA-C", &["D♯ 7ᵗʰ", "E♭ 7ᵗʰ"]),
    Chord::new(&["D#m", "Ebm"], "DFA", &["D♯ minor", "E♭ minor"]),
    Chord::new(&["D#m7", "Ebm7"], "DFA-C", &["D♯ minor 7ᵗʰ", "E♭ minor 7ᵗʰ"]),
    Chord::new(&["D#maj7", "Ebmaj7"], "DgA-d", &["D♯ raised 7ᵗʰ", "E♭ raised 7ᵗʰ"]),
    Chord::new(&["E"], "eGb", &["E"]),
    Chord::new(&["E7"], "eGb-d", &["E 7ᵗʰ"]),
    Chord::new(&["Emaj7"], "eGb-D", &["E raised 7ᵗʰ"]),
    Chord::new(&["Esus"], "eab", &["E suspended"]),
    Chord::new(&["E6"], "eGb-C", &["E 6ᵗʰ"]),
    Chord::new(&["Eadd2"], "eFGb", &["E added 2ⁿᵈ"]),
    Chord::new(&["Em"], "egb", &["E minor"]),
    Chord::new(&["Em7"], "egb-d", &["E minor 7ᵗʰ"]),
    Chord::new(&["Em7b5"], "egA-d", &["E half-diminished"]),
    Chord::new(&["F"], "fa-c", &["F"]),
    Chord::new(&["F7"], "fa-cD", &["F 7ᵗʰ"]),
    Chord::new(&["Fm"], "fG-c", &["F minor"]),
    Chord::new(&["Fm7"], "fG-cD", &["F minor 7ᵗʰ"]),
    Chord::new(&["Fm7b5"], "fGb-D", &["F half-diminished"]), 
    Chord::new(&["F#", "Gb"], "FA-C", &["F♯", "G♭"]),
    Chord::new(&["F#7", "Gb7"], "FA-Ce", &["F♯ 7ᵗʰ", "G♭ 7ᵗʰ"]),
    Chord::new(&["F#m", "Gbm"], "Fa-C", &["F♯ minor", "G♭ minor"]),
    Chord::new(&["F#m7", "Gbm7"], "Fa-Ce", &["F♯ minor 7ᵗʰ", "G♭ minor 7ᵗʰ"]),
    Chord::new(&["F#maj7", "Gbmaj7"], "FA-Cf", &["F♯ raised 7ᵗʰ", "G♭ raised 7ᵗʰ"]),
    Chord::new(&["G"], "gb-d", &["G"]),
    Chord::new(&["G7"], "gb-df", &["G 7ᵗʰ"]),
    Chord::new(&["Gmaj7"], "gb-dF", &["G raised 7ᵗʰ"]),
    Chord::new(&["Gsus"], "g-cd", &["G suspended"]),
    Chord::new(&["G6"], "gb-de", &["G 6ᵗʰ"]),
    Chord::new(&["Gadd2"], "gab-d", &["G added 2ⁿᵈ"]),
    Chord::new(&["Gm"], "gA-d", &["G minor"]),
    Chord::new(&["Gm7"], "gA-df", &["G minor 7ᵗʰ"]),
    Chord::new(&["Gm7b5"], "gA-Cf", &["G half-diminished"]),
    Chord::new(&["Gdim"], "gA-C", &["G diminished"]),
    Chord::new(&["Gdim7"],"gA-Ce", &["G diminished 7ᵗʰ"]),
    Chord::new(&["G+"], "gb-D", &["G augmented"]),
    Chord::new(&["G#", "Ab"], "G-cD", &["G♯", "A♭"]),
    Chord::new(&["G#7", "Ab7"], "G-cDF", &["G♯ 7ᵗʰ", "A♭ 7ᵗʰ"]),
    Chord::new(&["G#m", "Abm"], "Gb-D", &["G♯ minor", "A♭ minor"]),
    Chord::new(&["G#m7", "Abm7"], "Gb-DF", &["G♯ minor 7ᵗʰ", "A♭ minor 7ᵗʰ"]),
    Chord::new(&["G#maj7", "Abmaj7"], "G-cDg", &["G♯ raised 7ᵗʰ", "A♭ raised 7ᵗʰ"]),
    Chord::new(&["A"], "a-Ce", &["A"]),
    Chord::new(&["A7"], "a-Ceg", &["A 7ᵗʰ"]),
    Chord::new(&["Amaj7"], "a-CeG", &["A raised 7ᵗʰ"]),
    Chord::new(&["Asus"], "a-de", &["A suspended"]),
    Chord::new(&["A6"], "a-CeF", &["A 6ᵗʰ"]),
    Chord::new(&["Aadd2"], "ab-Ce", &["A added 2ⁿᵈ"]),
    Chord::new(&["Am"], "a-ce", &["A minor"]),
    Chord::new(&["Am7"], "a-ceg", &["A minor 7ᵗʰ"]),
    Chord::new(&["Am7b5"],  "a-cDg", &["A half-diminished"]),
    Chord::new(&["Adim"], "a-cD", &["A diminished"]),
    Chord::new(&["Adim7"], "a-cDF", &["A diminished 7ᵗʰ"]),
    Chord::new(&["A+"], "aCf", &["A augmented"]),
    Chord::new(&["A#", "Bb"], "A-df", &["A♯", "B♭"]),
    Chord::new(&["A#7", "Bb7"], "A-dfG", &["A♯ 7ᵗʰ", "B♭ 7ᵗʰ"]),
    Chord::new(&["A#m", "Bbm"], "A-Cf", &["A♯ minor", "B♭ minor"]),
    Chord::new(&["A#m7", "Bbm7"], "A-CfG", &["A♯ minor 7ᵗʰ", "B♭ minor 7ᵗʰ"]),
    Chord::new(&["A#maj7", "Bbmaj7"], "A-dfa", &["A♯ raised 7ᵗʰ", "B♭ raised 7ᵗʰ"]),
    Chord::new(&["B"], "b-DF", &["B"]),
    Chord::new(&["B7"], "b-DFa", &["B 7ᵗʰ"]),
    Chord::new(&["Bmaj7"], "b-DFA", &["B raised 7ᵗʰ"]),
    Chord::new(&["Bsus"], "b-eF", &["B suspended"]),
    Chord::new(&["B6"], "b-DFG", &["B 6ᵗʰ"]),
    Chord::new(&["Badd2"], "b-CDF", &["B added 2ⁿᵈ"]),
    Chord::new(&["Bm"], "b-dF", &["B minor"]),
    Chord::new(&["Bm7"], "b-dFa", &["B minor 7ᵗʰ"]),
    Chord::new(&["Bm7b5"], "b-dfa", &["B half-diminished"]),
    Chord::new(&["Bdim"], "b-df", &["B diminished"]),
    Chord::new(&["Bdim7"], "b-dfG", &["B diminished 7ᵗʰ"]),
    Chord::new(&["B+"], "b-Dg", &["B augmented"]),
    // slash / inversion chords, e.g.
    // A/C# is an A major chord with C# as the bass note or
    // A/E is an A major chord with E as the bass note
    // pianochord.org/c-major.html#hide1
    Chord::new(&["C/E"], "eg-c", &["C over E"]),
    Chord::new(&["C/G"], "g-ce", &["C over G"]),
    // pianochord.org/d-major.html#hide1
    Chord::new(&["D/F#", "D/Gb"], "Fa-d", &["D over F♯", "D over G♭"]),
    Chord::new(&["D/A"], "a-dF", &["D over A"]),
    // pianochord.org/e-major.html#hide1
    Chord::new(&["E/G#", "E/Ab"], "Gb-e", &["E over G♯", "E over A♭"]),
    Chord::new(&["E/B"], "b-eG", &["E over B"]),
    // pianochord.org/f-major.html#hide1
    Chord::new(&["F/A"], "a-cf", &["F over A"]),
    Chord::new(&["F/C"], "cfa", &["F over C"]),
    // pianochord.org/g-major.html#hide1
    Chord::new(&["G/B"], "b-dg", &["G over B"]),
    Chord::new(&["G/D"], "d-gb", &["G over B"]),
    // pianochord.org/a-major.html#hide1
    Chord::new(&["A/C#", "A/Db"], "Cea", &["A over C♯", "A over D♭"]),
    Chord::new(&["A/E"], "ea-C", &["A over E"]),
    // pianochord.org/b-major.html#hide1
    Chord::new(&["B/D#", "B/Eb"], "DFb", &["B over D♯", "B over E♭"]),
    Chord::new(&["B/F#", "B/Gb"], "Fb-D", &["B over F♯", "B over G♭"]),
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
    fn test_valid_chars() {
        for chord in ALL_CHORDS {
            for chr in chord.pattern.chars() {
                assert!(
                    "abcdefgACDFG-".contains(chr),
                    "chr={:?} pattern={:?}",
                    chr,
                    chord.pattern
                );
            }
        }
    }

    #[test]
    fn test_aliases() {
        for chord in ALL_CHORDS {
            assert_eq!(
                chord.short_names.len(),
                chord.names.len(),
                "Alias issue short_names={:?} names={:?}",
                chord.short_names,
                chord.names
            )
        }
    }
}
