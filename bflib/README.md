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

## Explaination
The brainfuck code
```brainfuck
brainfuck!(
    ++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[
        -<<<[
            ->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<
        ]>.>+[>>]>+
    ]
)
```
will be transpiled to the following rust code
```rust
::bflib::BrainfuckBlock::new(&|pc: &mut usize, mem: &mut Vec<u8>| {
    if mem.len() <= *pc {
        mem.append(&mut vec![0; *pc - mem.len() + 1]);
    }
    mem[*pc] = mem[*pc].wrapping_add(8);
    while mem[*pc] != 0 {
        *pc += 1;
        if mem.len() <= *pc {
            mem.append(&mut vec![0; *pc - mem.len() + 1]);
        }
        mem[*pc] = mem[*pc].wrapping_add(1);
        *pc += 1;
        if mem.len() <= *pc {
            mem.append(&mut vec![0; *pc - mem.len() + 1]);
        }
        mem[*pc] = mem[*pc].wrapping_add(4);
        if *pc < 2 {
            panic!("BrainFuck Program Counter reduced to below zero !!!");
        } else {
            *pc -= 2;
        }
        mem[*pc] = mem[*pc].wrapping_sub(1);
    }
    *pc += 1;
    if mem.len() <= *pc {
        mem.append(&mut vec![0; *pc - mem.len() + 1]);
    }
    mem[*pc] = mem[*pc].wrapping_add(2);
    *pc += 2;
    if mem.len() <= *pc {
        mem.append(&mut vec![0; *pc - mem.len() + 1]);
    }
    mem[*pc] = mem[*pc].wrapping_add(1);
    if *pc < 1 {
        panic!("BrainFuck Program Counter reduced to below zero !!!");
    } else {
        *pc -= 1;
    }
    while mem[*pc] != 0 {
        mem[*pc] = mem[*pc].wrapping_sub(1);
        while mem[*pc] != 0 {
            *pc += 2;
            if mem.len() <= *pc {
                mem.append(&mut vec![0; *pc - mem.len() + 1]);
            }
            mem[*pc] = mem[*pc].wrapping_add(1);
            if *pc < 2 {
                panic!("BrainFuck Program Counter reduced to below zero !!!");
            } else {
                *pc -= 2;
            }
            mem[*pc] = mem[*pc].wrapping_sub(1);
        }
        mem[*pc] = mem[*pc].wrapping_add(1);
        *pc += 2;
        if mem.len() <= *pc {
            mem.append(&mut vec![0; *pc - mem.len() + 1]);
        }
    }
    *pc += 1;
    if mem.len() <= *pc {
        mem.append(&mut vec![0; *pc - mem.len() + 1]);
    }
    mem[*pc] = mem[*pc].wrapping_add(1);
    while mem[*pc] != 0 {
        mem[*pc] = mem[*pc].wrapping_sub(1);
        if *pc < 3 {
            panic!("BrainFuck Program Counter reduced to below zero !!!");
        } else {
            *pc -= 3;
        }
        while mem[*pc] != 0 {
            mem[*pc] = mem[*pc].wrapping_sub(1);
            *pc += 1;
            if mem.len() <= *pc {
                mem.append(&mut vec![0; *pc - mem.len() + 1]);
            }
            while mem[*pc] != 0 {
                mem[*pc] = mem[*pc].wrapping_add(1);
                while mem[*pc] != 0 {
                    mem[*pc] = mem[*pc].wrapping_sub(1);
                }
                mem[*pc] = mem[*pc].wrapping_add(1);
                *pc += 1;
                if mem.len() <= *pc {
                    mem.append(&mut vec![0; *pc - mem.len() + 1]);
                }
                mem[*pc] = mem[*pc].wrapping_add(2);
                *pc += 3;
                if mem.len() <= *pc {
                    mem.append(&mut vec![0; *pc - mem.len() + 1]);
                }
                mem[*pc] = mem[*pc].wrapping_sub(1);
                if *pc < 2 {
                    panic!("BrainFuck Program Counter reduced to below zero !!!");
                } else {
                    *pc -= 2;
                }
            }
            if *pc < 1 {
                panic!("BrainFuck Program Counter reduced to below zero !!!");
            } else {
                *pc -= 1;
            }
            while mem[*pc] != 0 {
                if *pc < 1 {
                    panic!("BrainFuck Program Counter reduced to below zero !!!");
                } else {
                    *pc -= 1;
                }
            }
            *pc += 2;
            if mem.len() <= *pc {
                mem.append(&mut vec![0; *pc - mem.len() + 1]);
            }
            mem[*pc] = mem[*pc].wrapping_add(6);
            while mem[*pc] != 0 {
                if *pc < 2 {
                    panic!("BrainFuck Program Counter reduced to below zero !!!");
                } else {
                    *pc -= 2;
                }
                mem[*pc] = mem[*pc].wrapping_add(5);
                *pc += 2;
                if mem.len() <= *pc {
                    mem.append(&mut vec![0; *pc - mem.len() + 1]);
                }
                mem[*pc] = mem[*pc].wrapping_sub(1);
            }
            mem[*pc] = mem[*pc].wrapping_add(1);
            if *pc < 2 {
                panic!("BrainFuck Program Counter reduced to below zero !!!");
            } else {
                *pc -= 2;
            }
            mem[*pc] = mem[*pc].wrapping_add(2);
            print!("{}", mem[*pc] as char);
            while mem[*pc] != 0 {
                mem[*pc] = mem[*pc].wrapping_sub(1);
            }
            if *pc < 2 {
                panic!("BrainFuck Program Counter reduced to below zero !!!");
            } else {
                *pc -= 2;
            }
        }
        *pc += 1;
        if mem.len() <= *pc {
            mem.append(&mut vec![0; *pc - mem.len() + 1]);
        }
        print!("{}", mem[*pc] as char);
        *pc += 1;
        if mem.len() <= *pc {
            mem.append(&mut vec![0; *pc - mem.len() + 1]);
        }
        mem[*pc] = mem[*pc].wrapping_add(1);
        while mem[*pc] != 0 {
            *pc += 2;
            if mem.len() <= *pc {
                mem.append(&mut vec![0; *pc - mem.len() + 1]);
            }
        }
        *pc += 1;
        if mem.len() <= *pc {
            mem.append(&mut vec![0; *pc - mem.len() + 1]);
        }
        mem[*pc] = mem[*pc].wrapping_add(1);
    }
});
```