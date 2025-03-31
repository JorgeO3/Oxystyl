fn main() {
    // if let Err(e) = lalrpop::process_src() {
    //     eprintln!("fail lalrpop compilation: err - {e}")
    // }
    lalrpop::process_src().unwrap();
}
