fn main() {
    let mut output = String::new();

    for line in std::io::stdin().lines() {
        let input = line.unwrap();

        for c in input.chars() {
            if c == '\t' {
                output.push('\\');
                output.push('t');
            }  else if c == '\x08' {
                output.push('\\');
                output.push('b');
            } else {
                output.push(c);
            }
        }
        output.push('\\');
        output.push('n');
    }
    println!("{}", output);
}