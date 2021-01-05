const HI_BF: &str = include_str!("../../../hi.bf");

use super::{
    super::Parser,
    BrainfuckInstr,
    optimize
};

#[test]
fn test_hi_bf() {
    let mut parser = Parser::new();
    let mut code = parser
    .parse(HI_BF)
    .expect("Parser shat the bed: hi.bf should be a valid Brainfuck program.");
    optimize(&mut code);
    /* We haven't yet written the "virtual" instructions
    to expect repetitive patterns in hi.bf to be converted into. */
    unimplemented!()
}