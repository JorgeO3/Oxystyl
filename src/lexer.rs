use logos::Logos;

/// Representa todos los tokens que describe la gramática de Stylus CSS.
#[derive(Logos, Debug, PartialEq)]
pub enum Token<'source> {
    // ===============================================================
    // 1. Tokens de Control y Finalización
    // ===============================================================
    #[token("EOS")]
    Eos,

    // ===============================================================
    // 2. Palabras Reservadas / Keywords
    // ===============================================================
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("unless")]
    Unless,
    #[token("return")]
    Return,
    #[token("for")]
    For,
    #[token("mixin")]
    Mixin,
    #[token("in")]
    In,
    #[token("is not")]
    IsNot,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("isnt")]
    Isnt,
    #[token("is a")]
    IsA,
    #[token("is defined")]
    IsDefined,
    #[token("is")]
    Is,
    #[token("not")]
    Not,

    // ===============================================================
    // 3. Comentarios y Espacios
    // ===============================================================
    #[regex(r"//[^\n]*", callback = |_| ())]
    Comment, // Comentarios de línea (se ignora el contenido)
    #[regex(r"/\*[^*]*\*+([^/*][^*]*\*+)*/", callback = |_| ())]
    MultiLineComment,
    #[regex(r"\n")]
    Newline(&'source str), // Saltos de línea
    #[regex(r"[ \t]+")]
    Space, // Espacios y tabulaciones
    #[regex(r"\r", callback = |_| ())]
    Cr, // Carriage Return

    // ===============================================================
    // 4. Literales y Secuencias Especiales
    // ===============================================================
    #[regex(r"\\.[ \t]*")]
    Escaped(&'source str), // Secuencia de escape: '\' + cualquier carácter con espacios opcionales
    #[regex(r";[ \t]*")]
    Sep, // Separador: ';' seguido de espacios (opcional)
    #[regex(r"[\/:@\.;\?&=\*!,<>\#%0-9]+", priority = 1)]
    UrlChars(&'source str), // Secuencia de caracteres permitidos en URLs

    // Literales de cadena (dobles o simples)
    #[regex(r#""([^"\\]|\\.)*""#)]
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    String(&'source str),

    // Números (enteros, decimales y opcionalmente con unidades, p.ej., 10, 10px, .75rem)
    #[regex(r"(\d+(\.\d+)?|\.\d+)([a-zA-Z%]+)?")]
    Number(&'source str),

    // Colores en distintos formatos
    // Formato largo con alfa: #rrggbbaa
    #[regex(r"#[A-Fa-f0-9]{8}")]
    ColorRRGGBBAA(&'source str),
    // Formato largo sin alfa: #rrggbb
    #[regex(r"#[A-Fa-f0-9]{6}")]
    ColorRRGGBB(&'source str),
    // Formato corto con alfa: #rgba
    #[regex(r"#[A-Fa-f0-9]{4}")]
    ColorRGBA(&'source str),
    // Formato corto sin alfa: #rgb
    #[regex(r"#[A-Fa-f0-9]{3}")]
    ColorRGB(&'source str),
    // Formatos poco comunes: 2 dígitos o 1 dígito
    #[regex(r"#[A-Fa-f0-9]{2}")]
    ColorHex2(&'source str),
    #[regex(r"#[A-Fa-f0-9]{1}")]
    ColorHex1(&'source str),

    // ===============================================================
    // 5. Valores Primitivos
    // ===============================================================
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("null")]
    Null,

    // ===============================================================
    // 6. Delimitadores y Agrupadores
    // ===============================================================
    // Llaves
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    // Paréntesis
    #[token("(")]
    LParen,
    #[regex(r"\)")]
    RParen(&'source str),
    // Corchetes
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    // Otros delimitadores
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("..")]
    DotDot,
    #[token("...")]
    DotDotDot,
    #[token("$")]
    Dollar,
    #[token("#")]
    Hash,
    #[token("@")]
    At,

    // ===============================================================
    // 7. Operadores Aritméticos y Lógicos
    // ===============================================================
    // Operadores de suma, resta, multiplicación, división, etc.
    #[token("+")]
    Plus,
    #[token("+=")]
    PlusEq,
    #[token("-")]
    Minus,
    #[token("-=")]
    MinusEq,
    #[token("*")]
    Star,
    #[token("*=")]
    StarEq,
    #[token("/")]
    Slash,
    #[token("/=")]
    SlashEq,
    #[token("%")]
    Percent,
    #[token("%=")]
    PercentEq,
    #[token("**")]
    DoubleStar,
    // Operadores lógicos y de comparación
    #[token("!")]
    Exclamation, // Atención: se superpone con "!important" si no se ajusta el orden
    #[token("&")]
    Ampersand,
    #[token("&&")]
    DoubleAmpersand,
    #[token("||")]
    DoublePipe,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEq,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEq,
    #[token("=")]
    Eq,
    #[token("==")]
    EqEq,
    #[token("!=")]
    NotEq,
    #[token("~")]
    Tilde,
    #[token("?=")]
    QuestionEq,
    #[token(":=")]
    ColonEq,
    #[token("?")]
    Question,
    #[token(":")]
    Colon,

    // ===============================================================
    // 8. Identificadores
    // ===============================================================
    #[regex(r"[A-Za-z][A-Za-z_-]*")]
    Ident(&'source str),
}
