# BrainFuck compiler using Rust proc macro
More precisely, the BrainFuck-to-Rust transpiler using Rust proc macro

[![Crates.io](https://img.shields.io/crates/v/bflib)](https://crates.io/crates/bflib)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![docs.rs](https://docs.rs/rustpython/badge.svg)](https://docs.rs/bflib/)




## Examples:

1. Runs on `into` called
   (using `into` method to obtain `(pc: usize, mem: Vec<u8>)`)
   ```rust
   let (pc, mem) = brain_fuck!(
       ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
       >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
   ).into();
   println!("{:?}", (pc, mem));
   ```
2. Runs on `env` called
   (`env` method sets `pc` and `mem` for the block)
   (`env` method also returns `(pc: usize, mem: Vec<u8>)`)
   ```rust
   let mut pc = 0;
   let mut mem = vec![72, 101, 108, 108, 79, 119, 104, 97, 116, 65, 115, 10, 0];
   let (pc, mem) = brain_fuck!(
       [.>]
   ).env(pc, mem);
   println!("{:?}", (pc, mem));
   ```
3. Runs on drop
   ```rust
   brain_fuck!(
       ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
       >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
   );
   ```