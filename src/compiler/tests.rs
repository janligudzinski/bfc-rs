/// What an empty program should look like.
const ALLOCATE_AND_EXIT: &str = include_str!("alloc_exit.asm");
use super::compile;

#[test]
fn test_empty_program() {
    assert_eq!(compile(&[]), ALLOCATE_AND_EXIT)
}