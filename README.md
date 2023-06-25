# r6502
r6502 is a simple MOS6502 compiler.

## Usage
```
r6502 hello.asm
r6502 hello.asm hello.bin
r6502 hello.asm hex
r6502 hello.asm parse
```
## Commands
```
6502 assembly compiler

Usage: r6502.exe <FILE> [OUTPUT] [COMMAND]

Commands:
  hex    Print compiled hex values
  parse  Print parse result of the program
  help   Print this message or the help of the given subcommand(s)

Arguments:
  <FILE>    File path
  [OUTPUT]  Output path

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Todo
* Compile flag for NES rom
- segment
- program entry point
- export, include