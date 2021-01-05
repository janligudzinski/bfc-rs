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
    let mut total_loops = 0;
    let mut loop_no_stack = Vec::new();
    for instruction in code {
        // Generate the rest of the damn code.
        translate_instruction(
            instruction,
            &mut total_loops,
            &mut loop_no_stack,
            &mut output
        );
    }
    output.push_str(EXIT_SYSCALL);
    output
}
/// Transforms an individual Brainfuck instruction into an x86 one and stores it in the output buffer.
fn translate_instruction(instruction: &BrainfuckInstr, total_loops: &mut u32, loop_no_stack: &mut Vec<u32>, output: &mut String) {
    use BrainfuckInstr::*;
    output.push_str(match instruction {
        PointerDec => "dec rsi",
        PointerInc => "inc rsi",
        DataDec => "dec byte [rsi]",
        DataInc => "inc byte [rsi]",
        GetByte => "mov rdx,1\nmov rdi,0\nmov rax,0\nsyscall",
        PutByte => "mov rdx,1\nmov rdi,1\nmov rax,1\nsyscall",
        WhileNonzero => {
            *total_loops += 1;
            loop_no_stack.push(*total_loops);
            let asm = format!(
                "cmp byte [rsi],0\nje .end_{x}\n.while_{x}:\n",
                x = total_loops
            );
            output.push_str(&asm);
            return // early return so this block doesn't have to evaluate to an expression
        },
        EndWhile => {
            let asm = format!(
                "cmp byte [rsi],0\njne .while_{x}\n.end_{x}:\n",
                x = loop_no_stack.pop().expect("There should be a loop number on the stack at this point.")
            );
            output.push_str(&asm);
            return // ditto
        },
        /*We'll be writing out the full names of our enum's variants from now on
        to get proper type hints to show in our IDE, like (x: &u16).
        Rust-analyzer doesn't deal well with imported enum variants yet,
        unless we're talking about Options and Results - those are built-in. */
        BrainfuckInstr::PointerSub(x) => {
            unimplemented!()
        },
        BrainfuckInstr::PointerAdd(x) => {
            unimplemented!()
        },
        BrainfuckInstr::DataSub(x) => {
            unimplemented!()
        },
        BrainfuckInstr::DataAdd(x) => {
            unimplemented!()
        },
        BrainfuckInstr::Print(x) => {
            unimplemented!()
        }
    });
    output.push('\n');
}