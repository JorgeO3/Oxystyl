fn main() {
    if let Err(e) = lalrpop::process_root() {
        eprintln!("fail lalrpop compilation: err - {e}")
    }
}
