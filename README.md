# BrainFuck compiler using Rust proc macro
More precisely, the BrainFuck-to-Rust transpiler using Rust proc macro

[![Crates.io](https://img.shields.io/crates/v/bflib)](https://crates.io/crates/bflib)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![docs.rs](https://docs.rs/rustpython/badge.svg)](https://docs.rs/bflib/)




## Examples:

1. Hello World
   (run on dropping)
   ```rust
   brain_fuck!(
       ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
       >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
   );
   ```
2. using `into` method to obtain `(pc: usize, mem: Vec<u8>)` after running
   (run on `into` calling)
   ```rust
   let (pc, mem) = brain_fuck!(
       ++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[
           -<<<[
               ->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<
           ]>.>+[>>]>+
       ]
   ).into();
   println!("{:?}", (pc, mem));
   ```
3. use `env` method to set _Program Counter_ `pc` and _Memory_ `mem` for brainfuck codeblock
   (run on dropping)
   ```rust
   brain_fuck!(
       [.>]
   ).env(0, vec![79, 75, 10]);
   ```
4. Altogether
   (run on `into` calling)
   ```rust
   let (pc, mem) = brain_fuck!(
       [.>]
   ).env(0, vec![72, 101, 108, 108, 79, 119, 104, 97, 116, 65, 115, 10]).into();
   println!("{:?}", (pc, mem));
   ```
