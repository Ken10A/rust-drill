use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Ken10A <sample@example.com>")
        .about("Rust Echo")
        .get_matches();
    println!("{:?}", std::env::args());
}
