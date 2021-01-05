/// Module containing the parser code.
mod parser;
/// Module containing the compiler code.
mod compiler;
/// Type representing a standard or virtual Brainfuck instruction.
#[derive(Debug, PartialEq)] // this is for tests
pub enum BrainfuckInstr {
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
    EndWhile,
    /* The instructions below are our own virtual instructions that exist for optimization purposes.
    They DO NOT occur naturally in Brainfuck source code and have no corresponding text characters. */
    /// Subtract from the pointer.
    PointerSub(u16),
    /// Add to the pointer.
    PointerAdd(u16),
    /// Subtract from the current numer.
    DataSub(u8),
    /// Add to the current number.
    DataAdd(u8),
    /// Print a number of bytes to the standard output **at once**.
    Print(u16)
}

/// The syntax errors possible.
#[derive(Debug, PartialEq)]
pub enum SyntaxError {
    /// A closing square bracket was found at the contained line:index position, but there was no opening square bracket before it.
    PrematureEndWhile(usize, usize),
    /// The last while loop opened, at the contained line:index position, has no closing bracket.
    UnclosedWhile(usize, usize)
}

pub use compiler::compile;
pub use parser::Parser;