# calc-parser-rs

This is a Lexer (Logos) + Parser (Vanilla Rust) implementation for a simple calculator grammar.

**Features**: It supports bracketed expressions *,/,-,+ and floating point numbers (i.e. 123, 123.6, 123e-6 etc.).

**Limitations**: Currently negative numbers aren't supported (i.e. 4 * (-12)) won't work
