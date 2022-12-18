fn main() {
    if let Err(e) = cliwordle::get_args().and_then(cliwordle::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
