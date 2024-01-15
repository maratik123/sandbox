use sandbox::args_parser;

fn main() {
    let args = args_parser::parse().unwrap();
    println!("{args:?}");
}
