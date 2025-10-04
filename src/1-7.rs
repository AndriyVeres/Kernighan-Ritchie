fn main() {
    let mut input = String::new();

    /* read_line() returns string size if succeded. Iterating
     * out of string boundary produce panic from 1-6 */
    match std::io::stdin().read_line(&mut input) {
        Ok(size) => {
            if size >= 0 && size <= 1 {
                println!("Empty string");
            } else {
                println!("Input size:{} chars",size);
            }
        },
        Err(error) => println!("error: {error}"),
    };
}