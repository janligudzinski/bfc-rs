# BFC-RS

BFC-RS is a feature-complete [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) compiler for x86_64 Linux implemented in Rust. Follow its development at [my blog](https://oreganoli.github.io).

## Dependencies

BFC-RS requires `nasm` and `ld` to be available.
## Usage
```
bfc-rs <source_filename> [-o <output-filename>] [--dump-nasm] [--no-cleanup]
Options:
  -o, --output-filename
                    output filename (must be provided unless --dump-nasm is
                    explicitly passed)
  --dump-nasm       instead of compiling, print raw NASM output to stdout for
                    debugging
  --no-cleanup      do not clean up build directory after successful build
  --help            display usage information
```

## Licensing

By necessity, we're already using the word "fuck", so I see nothing wrong with licensing this code under the WTFPL.