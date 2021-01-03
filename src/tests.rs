const NEWLINE_PRINTING_PROGRAM: &str = include_str!("../newline.bf"); // same thing as we did with the NASM template while writing HWDPC
use super::*; // import all symbols from our parent module, the library root
use BrainfuckInstr::*;
#[test]
fn test_newline_program() {
    let mut parser = Parser;
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