fn main() {
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        for word in line.split_whitespace() {
            println!("{}", word);
        }
    }
}