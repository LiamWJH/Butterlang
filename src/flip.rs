mod lexer;

fn main() {
    println!("{:?}", lexer::lex("let a = 1 + 2 * 5\nwhile a < 13 { a +=1\n print(a) }"));
}
