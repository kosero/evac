mod tokenizer;
mod lexer;

use tokenizer::Tokenizer;
use lexer::Lexer;

fn main() {
    let input = "fn main(argc) { a = 1; return 42; }";

    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();

    println!("Tokens:");
    for token in &tokens {
        println!("{:?}", token);
    }

    let mut lexer = Lexer::new(tokens);
    if let Some(ast) = lexer.parse_expression() {
        println!("AST: {:?}", ast);
    } else {
        println!("Failed to parse expression.");
    }
}

