use logos::Logos;

/// Representa todos los tokens que describe la gramática.
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // (1) EOS: Fin de la entrada (sin outdent implícito aquí)
    #[token("EOS")]
    Eos,

    // (2) 'null'
    #[token("null")]
    Null,

    #[regex(r";")]
    Sep, // (3) Separador: ';'

    // (4) Palabras clave: if, else, unless, return, for, in
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
    #[token("in")]
    In,

    // (5) urlchars: secuencia de símbolos permitidos
    //    / : @ . ; ? & = * ! , < > # % 0-9
    #[regex(r"[\/:@\.;\?&=\*!,<>\#%0-9]+", priority = 1, callback = |lex| lex.slice().to_owned())]
    UrlChars(String),

    // (6) Comentario: línea que inicia con '//' hasta el final
    #[regex(r"//[^\n]*", callback = |_| ())]
    Comment, // Ignoramos el contenido en este ejemplo. Puedes guardarlo si deseas.

    #[regex(r"\n", callback = |lex| lex.slice().to_owned())]
    Newline(String),

    // (8) Carácter escapado: '\' + cualquier char, con espacios opcionales
    //    Para simplificar: `\\.` más cero o más espacios
    #[regex(r"\\.[ \t]*", callback = |lex| lex.slice().to_owned())]
    Escaped(String),

    // (9) !important con espacios opcionales
    #[regex(r"!important[ \t]*", callback = |lex| lex.slice().to_owned())]
    Important(String),

    // (10) Literal CSS, estilo @css { ... } (simplificado)
    #[regex(r"@css[ \t]*\{[^}]*\}[ \t]*", callback = |lex| lex.slice().to_owned())]
    Literal(String),

    // (11) Función anónima: '@('
    #[token("@(")]
    AnonFunc,

    // (12) Atrule: '@(-vendor-)?...'
    #[regex(r"@(-[A-Za-z]+-)?[A-Za-z0-9\-_]+", callback = |lex| lex.slice().to_owned())]
    AtRule(String),

    // (13) Function: identificador (opcionalmente con '-') seguido de '('
    #[regex(r"-?[A-Za-z_\$][A-Za-z0-9_\-\$]*\(", callback = |lex| lex.slice().to_owned())]
    Function(String),

    // (14) Llaves
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    // (15) Paréntesis: '(' | ')' + opcionales espacios
    #[token("(")]
    LParen,
    // Captura “) y espacios” juntos. Devolvemos la porción capturada.
    #[regex(r"\)", callback = |lex| lex.slice().to_owned())]
    RParen(String),

    // (16) Color: #rrggbbaa | #rrggbb | #rgba | #rgb | #nn | #n
    #[regex(r"#([A-Fa-f0-9]{8}|[A-Fa-f0-9]{6}|[A-Fa-f0-9]{4}|[A-Fa-f0-9]{3}|[A-Fa-f0-9]{2}|[A-Fa-f0-9]{1})",
        callback = |lex| lex.slice().to_owned())]
    Color(String),

    // (17) String: "..." o '...'
    //    Nota: en este patrón básico capturamos las comillas incluidas.
    #[regex(r#""([^"\\]|\\.)*""#, callback = |lex| lex.slice().to_owned())]
    #[regex(r#"'([^'\\]|\\.)*'"#, callback = |lex| lex.slice().to_owned())]
    String(String),

    #[regex(r"-?(\d+|\d*\.\d+)([A-Za-z]+|%)", callback = |lex| lex.slice().to_owned())]
    Unit(String), // (18) Unit: número (con o sin signo) + (letras o '%') -> p.ej: 10px, -5em, 0.5rem, 20%

    // (19) Operadores con nombre (namedop)
    #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("is")]
    Is,
    #[token("is not")]
    IsNot,
    #[token("isnt")]
    Isnt,
    #[token("is a")]
    IsA,
    #[token("is defined")]
    IsDefined,

    // (20) Boolean: true | false
    #[token("true")]
    True,
    #[token("false")]
    False,

    // (21) Unicode: U+[A-Fa-f0-9?]{1,6} [- [A-Fa-f0-9]{1,6}]?
    #[regex(r"U\+[A-Fa-f0-9\?]{1,6}(-[A-Fa-f0-9]{1,6})?", callback = |lex| lex.slice().to_owned())]
    Unicode(String),

    // (22) Ident: opcional '-' + ( letter | '_' | '$' ) + ...
    #[regex(r"-?[A-Za-z_\$][A-Za-z0-9_\-\$]*", callback = |lex| lex.slice().to_owned())]
    Ident(String),

    // (23) Operadores varios (op)
    #[token(",")]
    Comma,
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
    #[token("!")]
    Exclamation, // ¡Ojo! Se superpone con "!important" si no ajustamos el orden
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
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(".")]
    Dot,
    #[token("..")]
    DotDot,
    #[token("...")]
    DotDotDot,

    // (24) Espacio: uno o más espacios
    //     En Stylus, a veces interesa diferenciarlos (para indent/outdent).
    #[regex(r"[ \t]+")]
    Space,

    #[regex(r"^[a-zA-Z\-_]+$")]
    Label,
    // (25) Selector: '^' o secuencia de caracteres que
    //     no contenga salto de línea, coma o ';'
    //     Lo ponemos con la prioridad más baja para que no “robe” otros tokens.
    // #[regex(r"\^|[^\n,;]+", priority = 0, callback = |lex| lex.slice().to_owned())]
    // Selector(String),
}

const INPUT: &str = r##"
foo()
  current-property[0]

bar(args...)
  bar: args

body
  bar foo()

foo()
  baz()

bar()
  +baz()
    width: 10px

baz()
  test: current-property

raz
  foo: taz
  bar: taz

$global-padding ?= .75rem;

padding-left()
  {current-property[0]}: arguments

mixin($padding = $global-padding)
  padding-left: $padding * 2;

body
  mixin()
EOS
"##;

fn main() {
    // Un ejemplo de uso:

    let mut lexer = Token::lexer(INPUT);

    while let Some(tok) = lexer.next() {
        println!("{:?}: {:?}", tok.unwrap(), lexer.slice());
    }
}
