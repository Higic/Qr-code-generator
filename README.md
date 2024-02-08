# Qr-code-generator
A small terminal app for generating qr codes, build in Rust

## Use
Navigate to the directory and run it using `cargo run <url> [--png]`  
The `--png` argument generates a png-file with the qr code. The png filename is made from the input string, hashing it with SHA-256 algorithm. If no argument is given, the generator prints it out in ASCII to the terminal.
