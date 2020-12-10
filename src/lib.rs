//! Utility library for handling strings with german Umlauts "äöüÄÖÜßẞ"

pub trait UmlautsOwned {
    /// Like [`make_ascii_lowercase`] but it will also make utf8 umlauts lowercase:
    /// - 'Ä' -> 'ä'
    /// - 'Ö' -> 'ö'
    /// - 'Ü' -> 'ü'
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate umlauts;
    ///
    /// use umlauts::UmlautsOwned;
    ///
    /// let mut s = "Öl Ärmel Übermut".to_string();
    /// s.make_utf8_umlauts_lowercase();
    /// assert_eq!("öl ärmel übermut", s);
    /// ```
    fn make_utf8_umlauts_lowercase(&mut self);

    /// Like [`make_ascii_uppercase`] but it will also make utf8 umlauts lowercase:
    /// - 'Ä' -> 'ä'
    /// - 'Ö' -> 'ö'
    /// - 'Ü' -> 'ü'
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate umlauts;
    ///
    /// use umlauts::UmlautsOwned;
    ///
    /// let mut s = "Öl Ärmel Übermut".as_bytes().to_vec();
    /// s.as_mut_slice().make_utf8_umlauts_uppercase();
    /// assert_eq!("ÖL ÄRMEL ÜBERMUT".as_bytes(), s);
    /// ```
    fn make_utf8_umlauts_uppercase(&mut self);
}

impl UmlautsOwned for [u8] {
    fn make_utf8_umlauts_lowercase(self: &mut [u8]) {
        self.make_ascii_lowercase();
        let mut i = 0;
        while i < self.len() - 1 {
            let c = self[i];
            match (c, self[i+1]) {
                (b'A'..=b'Z', _) => self[i] = c.to_ascii_lowercase(),
                // Ä
                (0xc3, 0x84) => { eprintln!("foo"); self[i+1] = 0xa4 },
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
        self.make_ascii_uppercase();
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

#[cfg(feature = "unsafe")]
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
    #[cfg(feature = "unsafe")]
    fn make_utf8_string() {
        let mut text = "ÄÖÜäöüABCDabcd".to_string();
        text.make_utf8_umlauts_lowercase();
        assert_eq!(text, "äöüäöüabcdabcd");
        text.make_utf8_umlauts_uppercase();
        assert_eq!(text, "ÄÖÜÄÖÜABCDABCD");
    }
}
