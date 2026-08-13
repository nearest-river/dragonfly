


/*
    // Used quite often in relation to C ABI.
    pub const C: Symbol = ascii_letter_digit('C').unwrap();

    // RISC-V stuff
    #[expect(non_upper_case_globals)]
    pub const f: Symbol = ascii_letter_digit('f').unwrap();
    #[expect(non_upper_case_globals)]
    pub const d: Symbol = ascii_letter_digit('d').unwrap();

    /// Get the symbol for an integer.
    ///
    /// The first few non-negative integers each have a static symbol and therefore
    /// are fast.
    pub fn integer<N: TryInto<usize> + Copy + itoa::Integer>(n: N) -> Symbol {
        if let Result::Ok(idx) = n.try_into() {
            if idx < 10 {
                return Symbol::new(super::SYMBOL_DIGITS_BASE + idx as u32);
            }
        }
        let mut buffer = itoa::Buffer::new();
        let printed = buffer.format(n);
        Symbol::intern(printed)
    }

    pub const fn ascii_letter_digit(c: char) -> Option<Symbol> {
        let i = c as u32;
        Option::Some(Symbol::new(match c {
            '0'..='9' => super::SYMBOL_DIGITS_BASE + (i - '0' as u32),
            'A'..='Z' => super::SYMBOL_UPPERCASE_LETTERS_BASE + (i - 'A' as u32),
            'a'..='z' => super::SYMBOL_LOWERCASE_LETTERS_BASE + (i - 'a' as u32),
            _ => return Option::None,
        }))
    }

    pub fn character(c: char) -> Symbol {
        ascii_letter_digit(c).unwrap_or_else(|| {
            let mut buf: [u8; char::MAX_LEN_UTF8] = Default::default();
            Symbol::intern(c.encode_utf8(&mut buf))
        })
    }
*/


