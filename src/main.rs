mod lexer;

use lexer::*;

fn main() {
    loop {
        let mut input = String::new();
        while !input.contains("\n") {
            let next_line = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.push_str(next_line.as_str())
        }
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.by_ref().collect::<Vec<_>>();
        println!("lexing: {:?}", tokens)
    }
}
