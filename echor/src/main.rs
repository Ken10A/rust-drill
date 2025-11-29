fn main() {
    if let Err(e) = echor::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
