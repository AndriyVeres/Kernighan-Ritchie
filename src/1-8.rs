fn main()
{
    let mut blanks: u64 = 0;
    let mut tabs: u64 = 0;
    let mut lines: u64 = 0;
    let mut i: u64 = 0;

    for line in std::io::stdin().lines() {
        let str = line.unwrap();
        println!("[{}]:{}", i, str);

        lines += 1;

        for c in str.chars() {
            match c {
                ' ' => blanks += 1,
                '\t' => tabs += 1,
                _ => (),
            }
        }

        i += 1;
    }
    println!("blanks:{}, tabs:{}, newlines:{}", blanks, tabs, lines);
}