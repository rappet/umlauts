//! Utility library for handling strings with german Umlauts "äöüÄÖÜßẞ"
extern crate memchr;

/// Includes traits for manipulating slices/Strings/...
///
/// # Examples
/// ```rust
/// extern crate umlauts;
/// use umlauts::prelude::*;
/// ```
pub mod prelude;

/// Inplace string processing functions.
///
/// `UnlautsInplaceExt` adds inplace string processing functions for the german "Umlauts"
/// 'ä', 'ö', 'ü', 'ß' and their uppercase variants (except for uppercase 'ß').
/// Because these functions dont resize their containers or shift the containing data,
/// those methods are limited and should only be used if the higher performance
/// is absolutely needed.
pub trait UmlautsInplaceExt {
    /// Lowercases alphabetic ASCII chars and UTF-8 umlauts.
    ///
    /// Like [`make_ascii_lowercase`] but it will also make utf8 umlauts lowercase:
    /// - 'Ä' -> 'ä'
    /// - 'Ö' -> 'ö'
    /// - 'Ü' -> 'ü'
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate umlauts;
    /// use umlauts::prelude::*;
    ///
    /// let mut s = "Öl Ärmel Übermut".as_bytes().to_vec();
    /// s.make_utf8_umlauts_lowercase();
    /// assert_eq!("öl ärmel übermut".as_bytes(), s);
    /// ```
    ///
    /// [`make_ascii_lowercase`]: std::slice::[u8]::make_ascii_lowercase
    fn make_utf8_umlauts_lowercase(&mut self);

    /// Upercases alphabetic ASCII chars and UTF-8 umlauts.
    ///
    /// Like [`make_ascii_uppercase`] but it will also make utf8 umlauts uppercase:
    /// - 'ä' -> 'Ä'
    /// - 'ö' -> 'Ö'
    /// - 'ü' -> 'Ü'
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate umlauts;
    /// use umlauts::prelude::*;
    ///
    /// let mut s = "Öl Ärmel Übermut".as_bytes().to_vec();
    /// s.make_utf8_umlauts_uppercase();
    /// assert_eq!("ÖL ÄRMEL ÜBERMUT".as_bytes(), s);
    /// ```
    ///
    /// [`make_ascii_uppercase`]: std::slice::[u8]::make_ascii_uppercase
    fn make_utf8_umlauts_uppercase(&mut self);

    /// Converts Umlauts to ae, oe, ue, ss, ...
    ///
    /// Maps umlauts according to DIN 5007-2:
    /// - 'ä' -> 'ae'
    /// - 'ö' -> 'oe'
    /// - 'ü' -> 'ue'
    /// - 'Ä' -> 'Ae'
    /// - 'Ö' -> 'Oe'
    /// - 'Ü' -> 'Ue'
    /// - 'ß' -> 'ss'
    ///
    /// This function will ignore the uppercase ß,
    /// because it can't be mapped in place due to requiring
    /// three bytes.
    fn make_utf8_umlauts_to_ascii(&mut self);

    /// Converts Umlauts to lowercase ae, oe, ue, ss, ... and converts all ASCII characters to
    /// lowercase.
    ///
    /// Maps umlauts according to DIN 5007-2:
    /// - 'ä' -> 'ae'
    /// - 'ö' -> 'oe'
    /// - 'ü' -> 'ue'
    /// - 'Ä' -> 'ae'
    /// - 'Ö' -> 'oe'
    /// - 'Ü' -> 'ue'
    /// - 'ß' -> 'ss'
    ///
    /// This function will ignore the uppercase ß,
    /// because it can't be mapped in place due to requiring
    /// three bytes.
    fn make_utf8_umlauts_to_lowercase_ascii(&mut self);

    /// Converts Umlauts to caps AE, OE, UE, SS, ... and converts all ASCII characters to uppercase
    /// ASCII characters to uppercase.
    ///
    /// Maps umlauts according to DIN 5007-2:
    /// - 'ä' -> 'AE'
    /// - 'ö' -> 'OE'
    /// - 'ü' -> 'UE'
    /// - 'Ä' -> 'AE'
    /// - 'Ö' -> 'OE'
    /// - 'Ü' -> 'UE'
    /// - 'ß' -> 'SS'
    ///
    /// This function will ignore the uppercase ß,
    /// because it can't be mapped in place due to requiring
    /// three bytes.
    fn make_utf8_umlauts_to_uppercase_ascii(&mut self);
}

impl UmlautsInplaceExt for [u8] {
    fn make_utf8_umlauts_lowercase(self: &mut [u8]) {
        let mut i = 0;
        while i < self.len() - 1 {
            let c = self[i];
            match (c, self[i + 1]) {
                (b'A'..=b'Z', _) => self[i] = c.to_ascii_lowercase(),
                // Ä
                (0xc3, 0x84) => self[i + 1] = 0xa4,
                // Ö
                (0xc3, 0x96) => self[i + 1] = 0xb6,
                // Ü
                (0xc3, 0x9c) => self[i + 1] = 0xbc,
                _ => {}
            }
            i += 1;
        }
        if let Some(c) = self.last_mut() {
            c.make_ascii_lowercase()
        };
    }

    fn make_utf8_umlauts_uppercase(self: &mut [u8]) {
        let mut i = 0;
        while i < self.len() - 1 {
            let c = self[i];
            match (c, self[i + 1]) {
                (b'a'..=b'z', _) => self[i] = c.to_ascii_uppercase(),
                // ä
                (0xc3, 0xa4) => self[i + 1] = 0x84,
                // ö
                (0xc3, 0xb6) => self[i + 1] = 0x96,
                // ü
                (0xc3, 0xbc) => self[i + 1] = 0x9c,
                _ => {}
            }
            i += 1;
        }
        if let Some(c) = self.last_mut() {
            c.make_ascii_uppercase()
        };
    }

    fn make_utf8_umlauts_to_ascii(&mut self) {
        let mut i = 0;
        while i < self.len() - 1 {
            if let Some(next_i) = memchr::memchr(0xc3, &self[..self.len() - 1]) {
                if let Some(replacement) = match self[next_i + 1] {
                    0xa4 => Some((b'a', b'e')), // ae
                    0xb6 => Some((b'o', b'e')), // oe
                    0xbc => Some((b'u', b'e')), // ue
                    0x84 => Some((b'A', b'e')), // Ae
                    0x96 => Some((b'O', b'e')), // Oe
                    0x9c => Some((b'U', b'e')), // Ue
                    0x9f => Some((b's', b's')), // ss
                    _ => None,
                } {
                    self[next_i] = replacement.0;
                    self[next_i + 1] = replacement.1;
                    i = next_i + 1;
                } else {
                    i = next_i + 2;
                }
            } else {
                break;
            }
        }
    }

    fn make_utf8_umlauts_to_lowercase_ascii(&mut self) {
        let mut i = 0;
        while i < self.len() - 1 {
            if self[i] == 0xc3 {
                if let Some(replacement) = match self[i + 1] {
                    0xa4 => Some((b'a', b'e')), // ae
                    0xb6 => Some((b'o', b'e')), // oe
                    0xbc => Some((b'u', b'e')), // ue
                    0x84 => Some((b'a', b'e')), // AE
                    0x96 => Some((b'o', b'e')), // OE
                    0x9c => Some((b'u', b'e')), // UE
                    0x9f => Some((b's', b's')), // SS
                    _ => None,
                } {
                    self[i] = replacement.0;
                    self[i + 1] = replacement.1;
                    i += 1;
                } else {
                    i += 2;
                }
            } else {
                self[i].make_ascii_lowercase();
                i += 1;
            }
        }

        if let Some(byte) = self.last_mut() {
            byte.make_ascii_lowercase();
        }
    }

    fn make_utf8_umlauts_to_uppercase_ascii(&mut self) {
        let mut i = 0;
        while i < self.len() - 1 {
            if self[i] == 0xc3 {
                if let Some(replacement) = match self[i + 1] {
                    0xa4 => Some((b'A', b'E')), // AE
                    0xb6 => Some((b'O', b'E')), // OE
                    0xbc => Some((b'U', b'E')), // UE
                    0x84 => Some((b'A', b'E')), // AE
                    0x96 => Some((b'O', b'E')), // OE
                    0x9c => Some((b'U', b'E')), // UE
                    0x9f => Some((b'S', b'S')), // SS
                    _ => None,
                } {
                    self[i] = replacement.0;
                    self[i + 1] = replacement.1;
                    i += 1;
                } else {
                    i += 2;
                }
            } else {
                self[i].make_ascii_uppercase();
                i += 1;
            }
        }

        if let Some(byte) = self.last_mut() {
            byte.make_ascii_uppercase();
        }
    }
}

#[cfg(feature = "unsafe")]
impl UmlautsInplaceExt for str {
    fn make_utf8_umlauts_lowercase(&mut self) {
        unsafe {
            self.as_bytes_mut().make_utf8_umlauts_lowercase();
        }
    }

    fn make_utf8_umlauts_uppercase(&mut self) {
        unsafe {
            self.as_bytes_mut().make_utf8_umlauts_uppercase();
        }
    }

    fn make_utf8_umlauts_to_ascii(&mut self) {
        unsafe {
            self.as_bytes_mut().make_utf8_umlauts_to_ascii();
        }
    }

    fn make_utf8_umlauts_to_lowercase_ascii(&mut self) {
        unsafe {
            self.as_bytes_mut().make_utf8_umlauts_to_lowercase_ascii();
        }
    }

    fn make_utf8_umlauts_to_uppercase_ascii(&mut self) {
        unsafe {
            self.as_bytes_mut().make_utf8_umlauts_to_uppercase_ascii();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::UmlautsInplaceExt;

    #[test]
    fn char_length() {
        assert_eq!("ä".as_bytes().len(), 2);
        assert_eq!("ö".as_bytes().len(), 2);
        assert_eq!("ü".as_bytes().len(), 2);
        assert_eq!("Ä".as_bytes().len(), 2);
        assert_eq!("Ö".as_bytes().len(), 2);
        assert_eq!("Ü".as_bytes().len(), 2);
        assert_eq!("ß".as_bytes().len(), 2);
    }

    #[test]
    fn char_start() {
        assert_eq!("ä".as_bytes()[0], 0xc3);
        assert_eq!("ö".as_bytes()[0], 0xc3);
        assert_eq!("ü".as_bytes()[0], 0xc3);
        assert_eq!("Ä".as_bytes()[0], 0xc3);
        assert_eq!("Ö".as_bytes()[0], 0xc3);
        assert_eq!("Ü".as_bytes()[0], 0xc3);
        assert_eq!("ß".as_bytes()[0], 0xc3);
    }

    #[test]
    fn make_utf8_bytes() {
        let mut text = "ÄÖÜäöüABCDabcd".as_bytes().to_vec();
        text.make_utf8_umlauts_lowercase();
        assert_eq!(text, "äöüäöüabcdabcd".as_bytes());
        text.make_utf8_umlauts_uppercase();
        assert_eq!(text, "ÄÖÜÄÖÜABCDABCD".as_bytes());
    }

    #[test]
    fn make_utf8_umlauts_to_ascii_bytes() {
        let mut text = "ÄÖÜäöüABCDabcd".as_bytes().to_vec();
        text.make_utf8_umlauts_to_ascii();
        assert_eq!(text, "AeOeUeaeoeueABCDabcd".as_bytes());
    }

    #[test]
    fn make_utf8_umlauts_to_lowercase_ascii_bytes() {
        let mut text = "ÄÖÜäöüABCDabcd".as_bytes().to_vec();
        text.make_utf8_umlauts_to_lowercase_ascii();
        assert_eq!(text, "aeoeueaeoeueabcdabcd".as_bytes());
    }

    #[test]
    fn make_utf8_umlauts_to_uppercase_ascii_bytes() {
        let mut text = "ÄÖÜäöüABCDabcd".as_bytes().to_vec();
        text.make_utf8_umlauts_to_uppercase_ascii();
        assert_eq!(text, "AEOEUEAEOEUEABCDABCD".as_bytes());
    }

    #[test]
    #[cfg(feature = "unsafe")]
    fn make_utf8_string() {
        let mut text = "ÄÖÜäöüABCDabcd".to_string();
        text.make_utf8_umlauts_lowercase();
        assert_eq!(text, "äöüäöüabcdabcd");
        text.make_utf8_umlauts_uppercase();
        assert_eq!(text, "ÄÖÜÄÖÜABCDABCD");
    }
}
