use bfc_rs::{compile, Parser, SyntaxError};
use argh::FromArgs;
#[derive(FromArgs)]
/// BFC-RS v1.0.0
struct App {
    #[argh(positional, description = "source filename")]
    source_filename: String,
    #[argh(option, short = 'o', description = "output filename (must be provided unless --dump-nasm is explicitly passed)")]
    output_filename: Option<String>,
    #[argh(switch, description = "instead of compiling, print raw NASM output to stdout for debugging")]
    dump_nasm: bool,
    #[argh(switch, description = "do not clean up build directory after successful build")]
    no_cleanup: bool
}

fn main() {
    use std::process::{exit, Command};
    let opts: App = argh::from_env();
    let source_code: String = match std::fs::read_to_string(&opts.source_filename) {
        Ok(code) => code,
        Err(_) => {
            eprintln!("Error: Could not read the file {}.", &opts.source_filename);
            exit(1);
        }
    };
    let mut parser = Parser::new();
    let code = match parser.parse(&source_code) {
        Ok(code) => code,
        Err(e) => {
            match e {
                SyntaxError::UnclosedWhile(l, i) => {
                    eprintln!("Syntax error: unclosed while loop; [ without corresponding ] at {}:{}:{}.", &opts.source_filename, l, i);
                },
                SyntaxError::PrematureEndWhile(l, i) => {
                    eprintln!("Syntax error: closure of nonexistent while loop; ] without corresponding [ at {}:{}:{}.", &opts.source_filename, l, i);
                }
            }
            exit(1);
        }
    };
    let asm = compile(&code);
    if opts.dump_nasm {
        println!("{}", asm);
        return
    } else {
        if opts.output_filename.is_none() {
            eprintln!("Error: no output filename was provided.");
            exit(1);
        }
    }
    if std::fs::create_dir_all("build").is_err() {
        eprintln!("Error: Could not create build directory. Are you sure you have write permissions on this directory?");
        exit(1);
    }
    if std::fs::write("build/build.asm", &asm).is_err() {
        eprintln!("Error: Could not write NASM assembly. Are you sure you have write permissions on the build directory?");
        exit(1);
    }

    let assembler = match Command::new("nasm")
    .args(&["-f", "elf64", "build/build.asm", "-o", "build/build.o"])
    .status() {
        Ok(o) => o,
        Err(_) => {
            eprintln!("Error: Could not launch NASM assembler. Are you sure it's installed?");
            exit(1);
        }
    };
    if !assembler.success() {
        eprintln!("Error: NASM assembler exited with non-0 status code. This is likely an error in the generated code. Inspect build/build.asm to find out more.");
        exit(1);
    }
    let linker = match Command::new("ld")
    .args(&["-s", "-o", &opts.output_filename.expect("Output filename should have been checked by now"), "build/build.o"])
    .status() {
        Ok(o) => o,
        Err(_) => {
            eprintln!("Error: Could not launch `ld` linker. Are you sure it's installed?");
            exit(1);
        }
    };
    if !linker.success() {
        eprintln!("Error: `ld` linker exited with non-0 status code.");
        exit(1);
    }
    if !opts.no_cleanup {
        std::fs::remove_dir_all("build").expect("Expected to be able to remove build directory; was able to create it");
    }
}
