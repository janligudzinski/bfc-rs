use bfc_rs::{compile, Parser};
const HELLO_WORLD: &str = include_str!("../hello_world.bf");
fn main() {
    let mut parser = Parser::new();
    let code = match parser.parse(HELLO_WORLD) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Syntax error: {:?}", e);
            std::process::exit(1);
        }
    };
    let asm = compile(&code);
    println!("{}", asm);
}
