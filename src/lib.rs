/// Type representing a standard Brainfuck instruction.
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