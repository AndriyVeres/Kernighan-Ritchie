fn main() {
    let mut bar: Vec<usize> = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        for word in line.split_whitespace() {
            let validated_word
                = word.replace(&['.', ',', '!', '@', '#', '$', '%', '^', 
                                 '&', '*', '(', ')', '_', '+', '-', '=',
                                 '[', ']', '{', '}', ':', ';', '"', '\'',
                                 '?', '/', '.', ',', '<', '>', '|', '\\',][..], "");
            push_word_len(&mut bar, validated_word.len());
        }
    }
    print_horizon_hystogram(&bar);
}

fn push_word_len(dst: &mut Vec<usize>, val: usize) {
    if val == 0 {
        return;
    }

    if val > dst.len() {
        let mut length = dst.len();
        while length < val - 1 {
            dst.push(0); /* Push new 1 value */
            length += 1;
        }
        dst.push(1);
    } else {
        dst[val-1] = dst[val-1] + 1; /* Increment existing value */
    }
}

fn print_horizon_hystogram(src: &Vec<usize>) {
    let mut row = 0;
    for number in src.iter() {
        let mut bar = 0;
        row += 1;

        if *number == 0 {
            continue; /* Skip empty row */
        }
        print!("{}:", row);
        while bar < *number {
            print!("|");
            bar += 1;
        }
        println!("");
    }
}