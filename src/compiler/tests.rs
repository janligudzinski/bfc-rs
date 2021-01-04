/// What an empty program should look like.
const ALLOCATE_AND_EXIT: &str = include_str!("alloc_exit.asm");
const NEWLINE: &str = include_str!("newline.asm");
use super::compile;
use super::BrainfuckInstr;
use BrainfuckInstr::*;
#[test]
fn test_empty_program() {
    assert_eq!(compile(&[]), ALLOCATE_AND_EXIT)
}

#[test]
fn test_newline_program() {
    let code = vec![
        DataInc, DataInc, DataInc, DataInc, DataInc,
        DataInc, DataInc, DataInc, DataInc, DataInc,
        PutByte
    ];
    assert_eq!(compile(&code), NEWLINE)
}