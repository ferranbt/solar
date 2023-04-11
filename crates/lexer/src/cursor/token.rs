/// Parsed token.
///
/// It doesn't contain information about data that has been parsed, only the type of the token and
/// its size.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

// Size assertion.
const _: [(); 8] = [(); std::mem::size_of::<Token>()];

impl Token {
    #[inline]
    pub fn new(kind: TokenKind, len: u32) -> Self {
        Self { kind, len }
    }

    #[inline]
    pub fn eof() -> Self {
        Self::new(TokenKind::Eof, 0)
    }
}

/// Common lexeme types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Multi-char tokens:
    /// `// comment`
    ///
    /// `/// doc comment`
    LineComment { is_doc: bool },

    /// `/* block comment */`
    ///
    /// `/** block doc comment */`
    BlockComment { is_doc: bool, terminated: bool },

    /// Any whitespace character sequence.
    Whitespace,

    /// `ident` or `continue`
    ///
    /// At this step, keywords are also considered identifiers.
    Ident,

    /// Like the above, but containing invalid unicode codepoints.
    InvalidIdent,

    /// An unknown prefix, like `foo'`, `foo"`.
    ///
    /// Note that only the prefix (`foo`) is included in the token, not the separator (which is
    /// lexed as its own distinct token).
    UnknownPrefix,

    /// Examples: `123`, `0x123`, `hex"123"`. Note that `_` is an invalid
    /// suffix, but may be present here on string and float literals. Users of
    /// this type will need to check for and reject that case.
    ///
    /// See [LiteralKind] for more details.
    Literal { kind: LiteralKind },

    // One-char tokens:
    /// `;`
    Semi,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `[`
    OpenBracket,
    /// `]`
    CloseBracket,
    /// `~`
    Tilde,
    /// `?`
    Question,
    /// `:`
    Colon,
    /// `$`
    Dollar,
    /// `=`
    Eq,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `-`
    Minus,
    /// `&`
    And,
    /// `|`
    Or,
    /// `+`
    Plus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `^`
    Caret,
    /// `%`
    Percent,

    /// Unknown token, not expected by the lexer, e.g. `№`
    Unknown,

    /// End of input.
    Eof,
}

/// The literal types supported by the lexer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    /// `123`, `0x123`; empty_int: `0x`
    Int { base: Base, empty_int: bool },
    /// `123.321`, `1.2e3`; empty_int: `0x`
    Rational { base: Base, empty_exponent: bool },
    /// `"abc"`, `"abc`; `unicode"abc"`, `unicode"abc`
    Str { terminated: bool, unicode: bool },
    /// `hex"abc"`, `hex"abc`
    HexStr { terminated: bool },
}

/// Base of numeric literal encoding according to its prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    /// Literal doesn't contain a prefix.
    Decimal = 10,
    /// Literal starts with "0x".
    Hexadecimal = 16,
}
