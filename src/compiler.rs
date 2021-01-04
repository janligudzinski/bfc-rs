#[cfg(test)]
mod tests;
use super::BrainfuckInstr;
/// Instructions and directives found at the beginning of every program.
const ALLOCATE_AND_START: &str = include_str!("compiler/alloc_start.asm");
/// Ditto for the end.
const EXIT_SYSCALL: &str = include_str!("compiler/exit.asm");

/// Transforms abstract Brainfuck code into assembly.
pub fn compile(code: &[BrainfuckInstr]) -> String {
    let mut output = String::new();
    output.push_str(ALLOCATE_AND_START);
    for instruction in code {
        // Generate the rest of the damn code.
        output.push_str(translate_instruction(instruction));
        output.push('\n');
    }
    output.push_str(EXIT_SYSCALL);
    output
}
/// Transforms an individual Brainfuck instruction into an x86 one.
fn translate_instruction(instruction: &BrainfuckInstr) -> &str {
    use BrainfuckInstr::*;
    match instruction {
        DataInc => "inc byte [rsi]",
        PutByte => "mov rdx,1\nmov rdi,1\nmov rax,1\nsyscall",
        _ => unimplemented!()
    }
}