const HI_BF: &str = include_str!("../../../hi.bf");

use super::{
    super::Parser,
    BrainfuckInstr,
    optimize
};

#[test]
/// Tests the optimization of a representative Brainfuck program, "hi.bf".
fn test_hi_bf() {
    let mut parser = Parser::new();
    let mut code = parser
    .parse(HI_BF)
    .expect("Parser shat the bed: hi.bf should be a valid Brainfuck program.");
    optimize(&mut code);
    let expected_code: Vec<BrainfuckInstr> = vec![
        DataAdd(72),
        PointerInc,
        DataAdd(105),
        PointerInc,
        DataAdd(33),
        PointerInc,
        DataAdd(10),
        PointerInc,
        PointerSub(3),
        Print(4)
    ];
    assert_eq!(&code, &expected_code)
}

use BrainfuckInstr::*;
#[test]
/// Test for pointer arithmetic optimization.
fn test_ptr_arithmetic() {
    let mut code: Vec<BrainfuckInstr> = vec![
        PointerInc, PointerInc, PointerInc,
        PointerDec,
        PointerInc,
        PointerDec, PointerDec, PointerDec, PointerDec
    ];
    optimize(&mut code);
    let expected_code: Vec<BrainfuckInstr> = vec![
        PointerAdd(3),
        PointerDec,
        PointerInc,
        PointerSub(4)
    ];
    assert_eq!(&code, &expected_code)
}
#[test]
/// Test for data arithmetic optimization.
fn test_data_arithmetic() {
    let mut code: Vec<BrainfuckInstr> = vec![
        DataDec, DataDec,
        DataInc,
        PointerDec,
        Print(5),
        DataDec,
        DataInc, DataInc, DataInc
    ];
    optimize(&mut code);
    let expected_code: Vec<BrainfuckInstr> = vec![
        DataSub(2),
        DataInc,
        PointerDec,
        Print(5),
        DataDec,
        DataAdd(3)
    ];
    assert_eq!(&code, &expected_code)
}

#[test]
/// Test for printing optimization.
fn test_print() {
    let mut code: Vec<BrainfuckInstr> = vec![
        PutByte, PointerInc, PutByte,
        PointerDec,
        PutByte, PointerInc, PutByte, PointerInc, PutByte,
        PointerDec,
        PutByte
    ];
    optimize(&mut code);
    let expected_code: Vec<BrainfuckInstr> = vec![
        Print(2),
        PointerDec,
        Print(3),
        PointerDec,
        PutByte
    ];
    assert_eq!(&code, &expected_code)
}