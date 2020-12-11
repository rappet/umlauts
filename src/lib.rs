//! Utility library for handling strings with german Umlauts "äöüÄÖÜß"

/// Inplace string processing functions.
///
/// `UnlautsInplaceExt` adds inplace string processing functions for the german "Umlauts"
/// 'ä', 'ö', 'ü', 'ß' and their uppercase variants (except for uppercase 'ß').
/// Because these functions dont resize their containers or shift the containing data,
/// those methods are limited and should only be used if the higher performance
/// is absolutely needed.
pub trait UmlautsOwned {
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
    /// use umlauts::UmlautsOwned;
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
    /// use umlauts::UmlautsOwned;
    ///
    /// let mut s = "Öl Ärmel Übermut".as_bytes().to_vec();
    /// s.make_utf8_umlauts_uppercase();
    /// assert_eq!("ÖL ÄRMEL ÜBERMUT".as_bytes(), s);
    /// ```
    ///
    /// [`make_ascii_uppercase`]: std::slice::[u8]::make_ascii_uppercase
    fn make_utf8_umlauts_uppercase(&mut self);
}

impl UmlautsOwned for [u8] {
    fn make_utf8_umlauts_lowercase(self: &mut [u8]) {
        let mut i = 0;
        while i < self.len() - 1 {
            let c = self[i];
            match (c, self[i+1]) {
                (b'A'..=b'Z', _) => self[i] = c.to_ascii_lowercase(),
                // Ä
                (0xc3, 0x84) => self[i+1] = 0xa4,
                // Ö
                (0xc3, 0x96) => self[i+1] = 0xb6,
                // Ü
                (0xc3, 0x9c) => self[i+1] = 0xbc,
                _ => {},
            }
            i+=1;
        }
        self.last_mut().map(|c| c.make_ascii_lowercase());
    }

    fn make_utf8_umlauts_uppercase(self: &mut [u8]) {
        let mut i = 0;
        while i < self.len() - 1 {
            let c = self[i];
            match (c, self[i+1]) {
                (b'a'..=b'z', _) => self[i] = c.to_ascii_uppercase(),
                // ä
                (0xc3, 0xa4) => self[i+1] = 0x84,
                // ö
                (0xc3, 0xb6) => self[i+1] = 0x96,
                // ü
                (0xc3, 0xbc) => self[i+1] = 0x9c,
                _ => {},
            }
            i+=1;
        }
        self.last_mut().map(|c| c.make_ascii_uppercase());
    }
}

impl UmlautsOwned for str {
    fn make_utf8_umlauts_lowercase(&mut self) {
        unsafe { self.as_bytes_mut().make_utf8_umlauts_lowercase(); }
    }

    fn make_utf8_umlauts_uppercase(&mut self) {
        unsafe { self.as_bytes_mut().make_utf8_umlauts_uppercase(); }
    }
}

#[cfg(test)]
mod tests {
    use crate::UmlautsOwned;

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
    fn make_utf8_string() {
        let mut text = "ÄÖÜäöüABCDabcd".to_string();
        text.make_utf8_umlauts_lowercase();
        assert_eq!(text, "äöüäöüabcdabcd");
        text.make_utf8_umlauts_uppercase();
        assert_eq!(text, "ÄÖÜÄÖÜABCDABCD");
    }
}
