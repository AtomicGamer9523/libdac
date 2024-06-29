#![allow(missing_docs)]

use libmusic::*;

fn main() {
    println!("{:#?}", Chord::C_MAJOR);
    // fuzz();
}

fn fuzz() {
    for i in 0..=u8::MAX {
        let note = Note::try_from_raw(i);
        if let Ok(note) = note {
            println!("{note}");
        }
    }
}
