//! LibDAC
//! 
//! Digital Audio Commons Library

/// A Data structure that represents a note
///
/// Encoded as such:
/// 
/// ```text
/// | Note  | # | ♭ | Octave | 
/// | 0 0 0 | 0 | 0 | 0 0 0  |
/// ```
/// 
/// Note encoding:
/// 
/// ```text
/// 000 -> INVALID
/// 001 -> A
/// 010 -> B
/// 011 -> C
/// 100 -> D
/// 101 -> E
/// 110 -> F
/// 111 -> G
/// ```
/// 
/// Sharp encoding:
/// 
/// ```text
/// 0 -> Natural
/// 1 -> Sharp
/// ```
/// 
/// Flat encoding:
/// 
/// This should **NOT** be manually set.
/// 
/// ```text
/// 0 -> This is the true note
/// 1 -> This is an enharmonic equivalent
/// ```
/// 
/// Octave encoding:
/// 
/// ```text
/// 000 -> Lowest octave (1st)
/// 001 -> 2nd
/// 010 -> 3rd
/// 011 -> 4th
/// 100 -> 5th
/// 101 -> 6th
/// 110 -> 7th
/// 111 -> Highest octave (8th)
/// ```
#[repr(transparent)]
pub struct Note(u8);

/// A Data structure that represents a chord
/// 
/// It can be shifted to any octave
/// 
/// Encoded as such:
/// 
/// ```text
/// |      Note Encoding      | Octave | Type |
/// | 0 0 0 0 0 0 0 0 0 0 0 0 | 0 0 0  | 0    |
/// ```
/// 
/// Note encoding:
/// 
/// ```text
/// x0 -> A
/// x1 -> A#
/// x2 -> B
/// x3 -> C
/// x4 -> C#
/// x5 -> D
/// x6 -> D#
/// x7 -> E
/// x8 -> F
/// x9 -> F#
/// xA -> G
/// xB -> G#
/// ```
/// 
/// Octave encoding:
/// 
/// ```text
/// 000 -> Lowest octave (1st)
/// 001 -> 2nd
/// 010 -> 3rd
/// 011 -> 4th
/// 100 -> 5th
/// 101 -> 6th
/// 110 -> 7th
/// 111 -> Highest octave (8th)
/// ```
/// 
/// Type encoding:
/// 
/// The last bit is used to determine if the chord was defined
/// as a chord or used as an intermediary note.
/// Although it is not recommended to use this as a note, it is possible.
#[repr(transparent)]
pub struct Chord(u16);

mod validation;
mod consts;
mod traits;
