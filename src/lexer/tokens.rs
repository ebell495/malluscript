use std::fmt::Formatter;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType<'input> {
    Declaration,
    Write,
    InputString,
    InputNumber,
    LeftBrace,
    RightBrace,
    If,
    Else,
    Loop,
    Assignment,
    Plus,
    Minus,
    Product,
    Divide,
    Modulo,
    GreaterThan,
    LessThan,
    EqualTo,
    NotEqual,
    SemiColon,
    OpenParantheses,
    CloseParantheses,
    Um,
    Nekal,
    Literal(&'input str),
    Number(i64),
    Symbol(&'input str),
}

impl std::fmt::Display for TokenType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Declaration => write!(f, "ada_mwone"),
            TokenType::Write => write!(f, "dhe_pidicho"),
            TokenType::InputString => write!(f, "address_thada"),
            TokenType::InputNumber => write!(f, "number_thada"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::If => write!(f, "seriyano_mwone"),
            TokenType::Else => write!(f, "seri_allel"),
            TokenType::Loop => write!(f, "repeat_adi"),
            TokenType::Assignment => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Product => write!(f, "*"),
            TokenType::Divide => write!(f, "/"),
            TokenType::Modulo => write!(f, "%"),
            TokenType::GreaterThan => write!(f, "inekal_veluthane"),
            TokenType::LessThan => write!(f, "inekal_cheruthane"),
            TokenType::EqualTo => write!(f, "um_same_aane"),
            TokenType::NotEqual => write!(f, "um_same_alle"),
            TokenType::SemiColon => write!(f, ";"),
            TokenType::OpenParantheses => write!(f, "("),
            TokenType::CloseParantheses => write!(f, ")"),
            TokenType::Um => write!(f, "um"),
            TokenType::Nekal => write!(f, "ne_kal"),
            TokenType::Literal(literal) => write!(f, "{}", literal),
            TokenType::Number(number) => write!(f, "{}", number),
            TokenType::Symbol(symbol) => write!(f, "{}", symbol),
        }
    }
}
