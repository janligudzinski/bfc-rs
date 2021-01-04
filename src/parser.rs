#[cfg(test)]
mod tests;

use super::{BrainfuckInstr, SyntaxError};
/// Struct responsible for parsing Brainfuck.
struct Parser {
    /// The 1-indexed position (line number, position within line) of the earliest `[` without a matching `]`.
    earliest_unclosed: (usize, usize),
    /// The number that keeps track of whether or not the brackets are balanced so far.
    /// Opening a while loop with `[` bumps this number up by 1. Closing one with `]` drops it by 1.
    bracket_balance: isize /* In theory, there could be as many closing brackets as opening ones, and they could make up the whole program.
    Therefore, it makes sense to use `isize`, which can fit in half as big a number as `usize`, but either positive or negative.
    Remember that we want `bracket_balance` to end up at 0.*/
}
impl Parser {
    // Now that our struct isn't empty, we should make a constructor for it. Structs can be declared directly ad hoc, but only if the current module can see all of the struct's fields.
    // Rust has no language-level concept of a constructor; we just put a struct declaration into an ordinary function. Convention dictates we name it "new".
    fn new() -> Self {
        // "Self" is just a way to tell the compiler "put this data type's actual name here".
        // We could write "fn new() -> Parser", but this is neater and would require less searching-and-replacing if we wanted to manually rename `Parser`.
        Self {
            earliest_unclosed: (0, 0),
            bracket_balance: 0
        }
    }
    fn parse(&mut self, code: &str) -> Result<Vec<BrainfuckInstr>, SyntaxError> {
        use BrainfuckInstr::*;
        self.earliest_unclosed = (0, 0);
        self.bracket_balance = 0; // zero out the parser's state just in case
        let mut output = Vec::new();
        for (line_number, line) in code.lines().enumerate() {
            for (ch_number, ch) in line.chars().enumerate() {
                output.push(match ch {
                    '<' => PointerDec,
                    '>' => PointerInc,
                    '-' => DataDec,
                    '+' => DataInc,
                    ',' => GetByte,
                    '.' => PutByte,
                    '[' => {
                        // if we're not within (hopefully) a pair of braces already:
                        if self.bracket_balance == 0 {
                            self.earliest_unclosed = (line_number + 1, ch_number + 1);
                        }
                        self.bracket_balance += 1;
                        WhileNonzero // in Rust, code blocks are expressions that evaluate to the last expression within them
                        // this is why we rarely have to write "return" in functions
                    },
                    ']' => {
                        self.bracket_balance -= 1;
                        if self.bracket_balance < 0 {
                            // The moment we have one more ] than there have been [s, it no longer makes sense to parse the rest of the program.
                            // This is one of those situations where "return" is useful:
                            return Err(SyntaxError::PrematureEndWhile(line_number + 1, ch_number + 1))
                        }
                        EndWhile
                    },
                    _ => {
                        continue // skip this iteration if the character is something else
                    }
                });
            }
        }
        if self.bracket_balance == 0 {
            Ok(output)
        } else {
            // We've already returned the appropriate error if there was an extra ], so this is the only remaining possibility.
            Err(SyntaxError::UnclosedWhile(self.earliest_unclosed.0, self.earliest_unclosed.1))
        }
    }
}