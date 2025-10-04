fn main ()
{
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(size) => {
            for i in 0..size {
                /* Panic if input.as_bytes()[size]: the len is size but the index is size */
                let c: u8 = input.as_bytes()[i];
                match c {
                    b'\n' => println!("[{}]:{:>4} \'\\n\'", i, c),
                    _    => println!("[{}]:{:>4}  '{}'", i, c, c as char)
                }
            }
        },
        Err(e) => println!("String read fail, code:{}", e)
    }
}