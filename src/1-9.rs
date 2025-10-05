fn main() {
    let mut output = String::new();

    for line in std::io::stdin().lines() {
        let input = line.unwrap();
        let mut continuous_blanks: u64 = 0;

        for c in input.chars() {
            if c == ' ' {
                continuous_blanks += 1;
            } else {
                continuous_blanks = 0;
            }

            if continuous_blanks <= 1 {
                output.push(c);
            }
        }

        output.push('\n');
    }
    output.pop(); /* Remove '\n' from the output's end */
    println!("{}", output);
}