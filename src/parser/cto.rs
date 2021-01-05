#[cfg(test)]
mod tests;

use super::BrainfuckInstr;

/// Optimizes a list of Brainfuck instructions to be less repetitive.
pub fn optimize(code: &mut Vec<BrainfuckInstr>) {
    optimize_arithmetic(code);
}
/// Arithmetic optimization pass.
fn optimize_arithmetic(code: &mut Vec<BrainfuckInstr>) {
    use BrainfuckInstr::*;
    // new, optimized output:
    let mut opt = Vec::new(); /* Yes, we're cheating to keep the same function signature.
    We're going to just replace `code` with the new vector.
    We could optimize things in place, but I tried, believe me, I did. */
    // How many times the last instruction has been repeated.
    let mut repeats: u16 = 0;
    // Instruction last seen.
    let mut last_op = match code.get(0) {
        Some(op) => op.clone(),
        None => return // no instructions to optimize
    };
    let last = code.len() - 1;
    for (index, op) in code.iter().enumerate() {
        if *op == last_op {
            repeats += 1;
        }
        if *op != last_op || index == last {
            if repeats > 1 {
                match last_op {
                    DataDec | DataInc | PointerDec | PointerInc => {
                        opt.push(squash_arithmetic(&last_op, repeats));
                        repeats = 1;
                    },
                    _ => {
                        repeats = 1;
                    }
                }
            } else {
                opt.push(last_op.clone());
                repeats = 1;
            }
        }
        last_op = op.clone();
    }
    *code = opt;
}
/// "Compress" standard Brainfuck arithmetic operations repeated `x` times into our own virtual ones.
fn squash_arithmetic(op: &BrainfuckInstr, x: u16) -> BrainfuckInstr {
    use BrainfuckInstr::*;
    use std::cmp::min;
    let max_u16 = std::u16::MAX;
    match op {
        PointerDec => PointerSub(min(max_u16, x)),
        PointerInc => PointerAdd(min(max_u16, x)),
        DataDec => DataSub(min(255, x) as u8),
        DataInc => DataAdd(min(255, x) as u8),
        _ => {
            panic!("Tried to convert the non-arithmetic instruction {:?} into a virtual arithmetic instruction!", op)
        }
    }
}