fn main () {
    let mut frequencies = std::collections::HashMap::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        for word in line.split_whitespace() {
            for symbol in word.chars() {
                let character = frequencies.entry(symbol).or_insert(0);
                 *character += 1;
            }
        }
    }
    print_horizon_hystogram(&frequencies);
}

fn print_horizon_hystogram(src: &std::collections::HashMap<char, u64>) {
    for (symbol, number) in src {
        let mut bar = 0;
        print!("{}:", symbol);

        while bar < *number {
            print!("|");
            bar += 1;
        }
        println!("");
    }
}