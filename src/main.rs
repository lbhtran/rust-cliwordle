fn main() {
    if let Err(e) = wordlr::get_args().and_then(wordlr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
