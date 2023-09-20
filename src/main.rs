fn main() {
    if let Err(e) = rox::rox() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
