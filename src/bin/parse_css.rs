use cssparser::{Parser, ParserInput};

fn main() {
    let mut input = ParserInput::new("foo { bar: baz; }");
    let mut parser = Parser::new(&mut input);
    let token = parser.next();
    println!("{:?}", token);
}
