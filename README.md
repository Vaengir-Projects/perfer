# perfer
Small tool to measure performance and execution time

## Installation
```bash
cargo install perfer
```

## Usage
```
perfer [OPTIONS] <process>...

Arguments:
  <process>...  Command you wish to track

Options:
  -v, --verbose               Print stdout of process
  -G, --generate <generator>  [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                  Print help
  -V, --version               Print version
```
