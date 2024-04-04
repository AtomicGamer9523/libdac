use crate::*;

use core::fmt;

/// Generates a trap instruction to stop the program
#[inline]
const fn noop() -> ! {
    #[cfg(not(test))]
    unsafe { core::hint::unreachable_unchecked() }
    #[cfg(test)]
    panic!("This should be unreachable!")
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoteValidationError {
    InvalidStart,
    SharpAndFlat,
    ESharp,
    FFlat,
    BSharp,
    CFlat,
}

impl NoteValidationError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidStart => "Invalid Internal Note Representation: Started with '000'. That is not allowed!",
            Self::SharpAndFlat => "Invalid Internal Note Representation: Sharp and Flat cannot both be true!",
            Self::ESharp => "Invalid Internal Note Representation: E# is not allowed!",
            Self::FFlat => "Invalid Internal Note Representation: F♭ is not allowed!",
            Self::BSharp => "Invalid Internal Note Representation: B# is not allowed!",
            Self::CFlat => "Invalid Internal Note Representation: C♭ is not allowed!",
        }
    }
}

impl fmt::Display for NoteValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Note {
    /// Tries to create a note from a raw u8
    /// 
    /// Automatically validates the note
    pub const fn try_from_raw(raw: u8) -> Result<Self, NoteValidationError> {
        let note = Self(raw);
        match note.validate() {
            Ok(_) => Ok(note),
            Err(e) => Err(e),
        }
    }
    /// Tries to create a note from a raw u8
    /// 
    /// Does not validate the note
    pub const unsafe fn try_from_raw_unchecked(raw: u8) -> Self {
        Self(raw)
    }

    /// Gets the name of the note
    /// 
    /// Automatically validates the note
    pub const fn get_name_validated(&self) -> Result<&'static str, NoteValidationError> {
        let bits = match self.validate_inner() {
            Err(e) => return Err(e),
            Ok(b) => b,
        };
        Ok(match bits {
            0b001 => "A",
            0b010 => "B",
            0b011 => "C",
            0b100 => "D",
            0b101 => "E",
            0b110 => "F",
            0b111 => "G",
            // SAFETY: This is unreachable because the first 3 bits are validated
            _ => noop()
        })
    }
    /// Validates the note, to make sure that it is a valid note
    pub const fn validate(&self) -> Result<(), NoteValidationError> {
        if let Err(e) = self.validate_inner() {
            return Err(e);
        }
        Ok(())
    }
    #[must_use]
    #[doc(hidden)]
    const fn validate_inner(&self) -> Result<u8, NoteValidationError> {
        let first_3_bits: u8 = self.0 >> 5;
        let bit4: u8 = (self.0 >> 4) & 1;
        let bit5: u8 = (self.0 >> 3) & 1;
        // The first 3 bits cannot be 0b000
        if first_3_bits == 0b000 {
            return Err(NoteValidationError::InvalidStart);
        }
        // Sharp and Flat cannot both be true
        if bit4 == 1 && bit5 == 1 {
            return Err(NoteValidationError::SharpAndFlat);
        }
        // E# does not exist
        if first_3_bits == 0b101 && bit4 == 1 {
            return Err(NoteValidationError::ESharp);
        }
        // F♭ is enharmonically equivalent to E#,
        // Since E# does not exist, neither does F♭
        if first_3_bits == 0b101 && bit5 == 1 {
            return Err(NoteValidationError::FFlat);
        }
        // B# does not exist
        if first_3_bits == 0b010 && bit4 == 1 {
            return Err(NoteValidationError::BSharp);
        }
        // C♭ is enharmonically equivalent to B#,
        // Since B# does not exist, neither does C♭
        if first_3_bits == 0b010 && bit5 == 1 {
            return Err(NoteValidationError::CFlat);
        }
        Ok(first_3_bits)
    }
}