mod ast;
mod lexer;

use ast::AST;
use lexer::Lexer;

fn main() {
    let input = "fn main(argc) { a = 1; return 42; }";

    let mut lexer = Lexer::new(input);
    let lex = lexer.lex();

    println!("Tokens:");
    for token in &lex {
        println!("{:?}", token);
    }

    let mut ast = AST::new(lex);
    if let Some(ast_text) = ast.parse_expression() {
        println!("AST: {:?}", ast_text);
    } else {
        println!("Failed to parse expression.");
    }
}
