mod tokenizer;

use tokenizer::Tokenizer;

fn main() {
    let input = "db x = 42 + 8;";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
}
