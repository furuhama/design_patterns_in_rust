// Interpreter pattern
//
// Define simple DSL string literal, and parse & execute it.

pub fn interpreter() {
    Interpreter::parse_and_execute("3 Mew Waon !");
}

struct Interpreter;

impl Interpreter {
    fn parse_and_execute(s: &str) {
        if let Some(i) = s.find(' ') {
            let (num, word) = s.split_at(i); // => ("3", " Mew Waon !")
            let word = &word[1..]; // Trim first blank from `word`.
            let times = num.parse::<usize>().unwrap();
            for _ in 0..times {
                print!("{} ", word);
            }
            println!();
        }
    }
}
