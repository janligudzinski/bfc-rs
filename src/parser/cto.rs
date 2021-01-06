#[cfg(test)]
mod tests;

use super::BrainfuckInstr;

/// Optimizes a list of Brainfuck instructions to be less repetitive.
pub fn optimize(code: &mut Vec<BrainfuckInstr>) {
    optimize_arithmetic(code);
    // dbg!(&code);
    /* this helped us while fixing
    a buggy interaction between the arithmetic and print passes
    the reason was we didn't push the last instruction
    to the instruction vector if it wasn't arithmetic 🤷
    see lines 119-122 */
    optimize_printing(code);
}

fn optimize_printing(code: &mut Vec<BrainfuckInstr>) -> () {
    use BrainfuckInstr::{PointerInc, PutByte, Print};
    let mut last_op = match code.get(0) {
        Some(op) => op.clone(),
        None => return // no instructions to optimize
    };
    let mut print_lvl = 0u16;
    let mut opt = Vec::new();
    for op in code.iter() {
        match op {
            PutByte => {
                if print_lvl == 0 {
                    print_lvl += 1;
                } else {
                    match last_op {
                        PointerInc  => print_lvl += 1,
                        _ => {
                            opt.push(PutByte);
                            print_lvl = 1
                        }
                    }
                }
            },
            PointerInc => {
                match last_op {
                    PutByte => (),
                    _ => {
                        opt.push(PointerInc);
                        print_lvl = 0;
                    }
                }
            }
            other => {
                match print_lvl {
                    0 => {
                        opt.push(other.clone());
                        print_lvl = 0;
                    },
                    1 => {
                        opt.push(PutByte);
                        opt.push(other.clone());
                        print_lvl = 0;
                    },
                    n => {
                        opt.push(Print(n));
                        opt.push(other.clone());
                        print_lvl = 0;
                    }
                }
            }
        }
        last_op = op.clone();
    }
    match print_lvl {
        0 => (),
        1 => {
            opt.push(PutByte);
        },
        n => {
            opt.push(Print(n));
        }
    }
    *code = opt;
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
    match last_op {
        DataDec | DataInc | PointerDec | PointerInc => (),
        _ => opt.push(last_op)   
    };
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