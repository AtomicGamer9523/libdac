use crate::*;

use core::cmp;
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
            }
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

impl fmt::Debug for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_a  = (self.0 >> 0xF) & 1 == 1;
        let is_as = (self.0 >> 0xE) & 1 == 1;
        let is_b  = (self.0 >> 0xD) & 1 == 1;
        let is_c  = (self.0 >> 0xC) & 1 == 1;
        let is_cs = (self.0 >> 0xB) & 1 == 1;
        let is_d  = (self.0 >> 0xA) & 1 == 1;
        let is_ds = (self.0 >> 0x9) & 1 == 1;
        let is_e  = (self.0 >> 0x8) & 1 == 1;
        let is_f  = (self.0 >> 0x7) & 1 == 1;
        let is_fs = (self.0 >> 0x6) & 1 == 1;
        let is_g  = (self.0 >> 0x5) & 1 == 1;
        let is_gs = (self.0 >> 0x4) & 1 == 1;
        f.debug_struct("Chord")
            .field("a", &is_a)
            .field("a#", &is_as)
            .field("b", &is_b)
            .field("c", &is_c)
            .field("c#", &is_cs)
            .field("d", &is_d)
            .field("d#", &is_ds)
            .field("e", &is_e)
            .field("f", &is_f)
            .field("f#", &is_fs)
            .field("g", &is_g)
            .field("g#", &is_gs)
            .finish()
    }
}

macro_rules! raw_num {
    ($($t:ty),+) => ($(
        impl fmt::Binary for $t {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl fmt::LowerHex for $t {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl fmt::UpperHex for $t {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl fmt::Octal for $t {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl fmt::LowerExp for $t {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl fmt::UpperExp for $t {
            #[inline]
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
            }
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
