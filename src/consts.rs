use crate::*;

impl Chord {
    /// C Major
    /// 
    /// C D E F G A B
    pub const C_MAJOR: Chord = Chord(0b_10_1_10_10_1_10_10_011_0);
    /// C Minor
    /// 
    /// C D E♭ F G A♭ B♭
    pub const C_MINOR: Chord = Chord(0b_01_0_10_11_0_10_11_011_1);
}

impl Note {
    /// A
    pub const A:  Note = Note(0b001_0_0_011);
    /// A#
    pub const AS: Note = Note(0b001_1_0_011);
    /// A♭ (G#)
    pub const AF: Note = Note(0b111_0_1_011);
    /// B
    pub const B:  Note = Note(0b010_0_0_011);
    /// B♭ (A#)
    pub const BF: Note = Note(0b001_0_1_011);
    /// C
    pub const C:  Note = Note(0b011_0_0_011);
    /// C#
    pub const CS: Note = Note(0b011_1_0_011);
    /// D
    pub const D:  Note = Note(0b100_0_0_011);
    /// D#
    pub const DS: Note = Note(0b100_1_0_011);
    /// D♭ (C#)
    pub const DF: Note = Note(0b011_0_1_011);
    /// E
    pub const E:  Note = Note(0b101_0_0_011);
    /// E♭ (D#)
    pub const EF: Note = Note(0b100_0_1_011);
    /// F
    pub const F:  Note = Note(0b110_0_0_011);
    /// F#
    pub const FS: Note = Note(0b110_1_0_011);
    /// G
    pub const G:  Note = Note(0b111_0_0_011);
    /// G#
    pub const GS: Note = Note(0b111_1_0_011);
    /// G♭ (F#)
    pub const GF: Note = Note(0b110_0_1_011);
}
