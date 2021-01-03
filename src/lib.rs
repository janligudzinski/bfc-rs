#[cfg(test)]
mod tests;
/// Type representing a standard Brainfuck instruction.
#[derive(Debug, PartialEq)] // this is for tests
enum BrainfuckInstr {
    /// Move the data pointer back one cell.
    PointerDec,
    /// Move the data pointer forward one cell.
    PointerInc,
    /// Decrement the value of the current memory cell.
    DataDec,
    /// Increment the value of the current memory cell.
    DataInc,
    /// Get a byte from the standard input and store it in the current memory cell.
    GetByte,
    /// Write the current memory cell's value to the standard output.
    PutByte,
    /// Begin a while loop conditional on the current value not being zero.
    WhileNonzero,
    /// Close the while loop.
    EndWhile
}
/// The syntax errors possible.
#[derive(Debug, PartialEq)]
enum SyntaxError {
    /// A closing square bracket was found at the contained line:index position, but there was no opening square bracket before it.
    PrematureEndWhile(usize, usize),
    /// The last while loop opened, at the contained line:index position, has no closing bracket.
    UnclosedWhile(usize, usize)
}
/// Struct responsible for parsing Brainfuck.
struct Parser; // empty because it has no internal state yet
impl Parser {
    fn parse(&mut self, code: &str) -> Result<Vec<BrainfuckInstr>, SyntaxError> {
        use BrainfuckInstr::*;
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
                    '[' => WhileNonzero,
                    ']' => EndWhile,
                    _ => { continue; /* skip this iteration if the character is something else */}
                });
            }
        }
        Ok(output)
    }
}