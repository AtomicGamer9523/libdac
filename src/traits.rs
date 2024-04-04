use crate::*;

use core::fmt;

const PRE: &str = "\x1b[31m[\x1b[91mNOTE FORMAT ERROR\x1b[31m] ";
const POST: &str = "\x1b[0m";

impl fmt::Debug for Note {
    /// Debugs the note
    /// 
    /// # Safety
    /// 
    /// This function will automatically validate the note.
    /// If you use print! or format! macros, it may panic if the note is invalid.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.get_name_validated() {
            Err(e) => {
                writeln!(f, "{PRE}{e}{POST}")?;
                return Err(fmt::Error);
            },
            Ok(n) => n,
        };
        let is_sharp = (self.0 >> 4) & 1 == 1;
        let is_flat = (self.0 >> 3) & 1 == 1;
        let octave = self.0 & 0b111;
        f.debug_struct("Note")
            .field("note", &name)
            .field("sharp", &is_sharp)
            .field("flat", &is_flat)
            .field("octave", &octave)
            .finish()
    }
}

macro_rules! raw_num {
    ($($t:ty),+) => ($(
        impl fmt::Binary for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl fmt::LowerHex for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl fmt::UpperHex for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl fmt::Octal for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl fmt::LowerExp for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl fmt::UpperExp for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    )+);
}

raw_num!(Note, Chord);

impl fmt::Display for Note {
    /// Display the note as a string
    /// 
    /// # Safety
    /// 
    /// This function will automatically validate the note.
    /// If you use print! or format! macros, it may panic if the note is invalid.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.get_name_validated() {
            Err(e) => {
                writeln!(f, "{PRE}{e}{POST}")?;
                return Err(fmt::Error);
            },
            Ok(n) => n,
        };
        let is_sharp = (self.0 >> 4) & 1 == 1;
        let is_flat = (self.0 >> 3) & 1 == 1;
        let symbol = if is_sharp {
            "#"
        } else if is_flat {
            "♭"
        } else {
            ""
        };
        write!(f, "{name}{symbol}")?;

        Ok(())
    }
}
