pub mod lexer;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Int(i64),
    Float(f64),
    String(&'a str),
    UnaryOperation(UnaryOperator, Box<Spanned<Expr<'a>>>),
    BinaryOperation(
        BinaryOperator,
        Box<Spanned<Expr<'a>>>,
        Box<Spanned<Expr<'a>>>,
    ),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Minus,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

// Lexer simplificado que maneja los Span internamente
use logos::{Logos, SpannedIter};

#[derive(Debug)]
pub enum LexicalError {
    InvalidToken,
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token<'input> {
    #[regex(r"\d+", |lex| lex.slice())]
    Int(&'input str),

    #[regex(r"\d+\.\d+", |lex| lex.slice())]
    Float(&'input str),

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,
}

// Un lexer que proporciona tokens con posición a LALRPOP
pub struct Lexer<'input> {
    token_stream: SpannedIter<'input, Token<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

// Tipo compatible con LALRPOP
pub type SpannedToken<'input> = (usize, Token<'input>, usize);

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<SpannedToken<'input>, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| match token {
            Token::Error => Err(LexicalError::InvalidToken),
            token => Ok((span.start, token, span.end)),
        })
    }
}

lalrpop_util::lalrpop_mod!(pub parser);
