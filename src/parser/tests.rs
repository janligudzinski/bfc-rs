const NEWLINE_PRINTING_PROGRAM: &str = include_str!("../../newline.bf"); // same thing as we did with the NASM template while writing HWDPC
const PREMATURE_PROGRAM: &str = include_str!("../../premature.bf");
const UNMATCHED_PROGRAM: &str = include_str!("../../unmatched.bf");
use super::*; // import all symbols from our parent module, the parser module
#[test]
fn test_newline_program() {
    /* refer to all variants of the instruction type directly,
    without prefixing them with "BrainfuckInstr::" */
    use BrainfuckInstr::*;
    let mut parser = Parser::new();
    let output = parser
    .parse(NEWLINE_PRINTING_PROGRAM)
    .expect("The newline program contains no syntax errors, something is wrong with the parser.");
    let expected_output: Vec<BrainfuckInstr> = vec![
        DataInc, DataInc, DataInc, DataInc, DataInc,
        DataInc, DataInc, DataInc, DataInc, DataInc,
        PutByte
    ];
    assert_eq!(&output, &expected_output)
}
#[test]
fn test_premature_program() {
    let mut parser = Parser::new();
    let output = parser
    .parse(PREMATURE_PROGRAM)
    .expect_err("The code in premature.bf shouldn't produce a valid instruction list.");
    assert_eq!(output, SyntaxError::PrematureEndWhile(2, 1)) // expect an error on line 2, character 1
}
#[test]
fn test_unmatched_program() {
    let mut parser = Parser::new();
    let output = parser
    .parse(UNMATCHED_PROGRAM)
    .expect_err("The code in unmatched.bf shouldn't produce a valid instruction list.");
    assert_eq!(output, SyntaxError::UnclosedWhile(1, 8)) // expect an error or line 1, character 8
}