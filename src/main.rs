fn main() {
    let expr = oxystyl::parser::ExprParser::new()
        .parse("10 + 20 * 30")
        .unwrap();

    println!("{:#?}", expr);
}
