fn main() {
    let args: Vec<String> = std::env::args().collect();
    std::fs::write("/tmp/argv.txt", args.join(" ")).unwrap();
}
