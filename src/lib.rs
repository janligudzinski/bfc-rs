/// Module containing the parser code.
mod parser;
/// Module containing the compiler code.
mod compiler;
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
pub enum SyntaxError {
    /// A closing square bracket was found at the contained line:index position, but there was no opening square bracket before it.
    PrematureEndWhile(usize, usize),
    /// The last while loop opened, at the contained line:index position, has no closing bracket.
    UnclosedWhile(usize, usize)
}