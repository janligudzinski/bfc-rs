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
    let mut bracket_counter = 0;
    for instruction in code {
        // Generate the rest of the damn code.
        translate_instruction(instruction, &mut bracket_counter, &mut output);
    }
    output.push_str(EXIT_SYSCALL);
    output
}
/// Transforms an individual Brainfuck instruction into an x86 one and stores it in the output buffer.
fn translate_instruction(instruction: &BrainfuckInstr, bracket_counter: &mut u32, output: &mut String) {
    use BrainfuckInstr::*;
    output.push_str(match instruction {
        PointerDec => "dec rsi",
        PointerInc => "inc rsi",
        DataDec => "dec byte [rsi]",
        DataInc => "inc byte [rsi]",
        GetByte => "mov rdx,1\nmov rdi,0\nmov rax,0\nsyscall",
        PutByte => "mov rdx,1\nmov rdi,1\nmov rax,1\nsyscall",
        WhileNonzero=> {
            let asm = format!(
                "cmp [rsi],0\nje end_{x}\nwhile_{x}:",
                x = bracket_counter
            );
            *bracket_counter += 1;
            output.push_str(&asm);
            return // early return so this block doesn't have to evaluate to an expression
        },
        EndWhile => {
            *bracket_counter -= 1;
            let asm = format!(
                "cmp [rsi],0\njne while_{x}\nend_{x}:",
                x = bracket_counter
            );
            output.push_str(&asm);
            return // ditto
        }
    });
    output.push('\n');
}