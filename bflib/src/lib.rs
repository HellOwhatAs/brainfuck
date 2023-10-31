//! # BrainFuck compiler using Rust proc macro
//! More precisely, the BrainFuck-to-Rust transpiler using Rust proc macro
//! 
//! ## Examples:
//! 
//! 1. Hello World
//!    (run on dropping)
//!    ```rust
//!    brain_fuck!(
//!        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
//!        >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
//!    );
//!    ```
//! 2. using `into` method to obtain `(pc: usize, mem: Vec<u8>)` after running
//!    (run on `into` calling)
//!    ```rust
//!    let (pc, mem) = brain_fuck!(
//!        ++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[
//!            -<<<[
//!                ->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<
//!            ]>.>+[>>]>+
//!        ]
//!    ).into();
//!    println!("{:?}", (pc, mem));
//!    ```
//! 3. use `env` method to set _Program Counter_ `pc` and _Memory_ `mem` for brainfuck codeblock
//!    (run on dropping)
//!    ```rust
//!    brain_fuck!(
//!        [.>]
//!    ).env(0, vec![79, 75, 10]);
//!    ```
//! 4. Altogether
//!    (run on `into` calling)
//!    ```rust
//!    let (pc, mem) = brain_fuck!(
//!        [.>]
//!    ).env(0, vec![72, 101, 108, 108, 79, 119, 104, 97, 116, 65, 115, 10]).into();
//!    println!("{:?}", (pc, mem));
//!    ```
//! 
//! 
//! 
//! ## Explaination
//! The brainfuck code
//! ```brainfuck
//! brainfuck!(
//!     ++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[
//!         -<<<[
//!             ->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<
//!         ]>.>+[>>]>+
//!     ]
//! )
//! ```
//! will be transpiled to the following rust code
//! ```rust
//! ::bflib::BrainfuckBlock::new(&|pc: &mut usize, mem: &mut Vec<u8>| {
//!     if mem.len() <= *pc {
//!         mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!     }
//!     mem[*pc] = mem[*pc].wrapping_add(8);
//!     while mem[*pc] != 0 {
//!         *pc += 1;
//!         if mem.len() <= *pc {
//!             mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!         }
//!         mem[*pc] = mem[*pc].wrapping_add(1);
//!         *pc += 1;
//!         if mem.len() <= *pc {
//!             mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!         }
//!         mem[*pc] = mem[*pc].wrapping_add(4);
//!         if *pc < 2 {
//!             panic!("BrainFuck Program Counter reduced to below zero !!!");
//!         } else {
//!             *pc -= 2;
//!         }
//!         mem[*pc] = mem[*pc].wrapping_sub(1);
//!     }
//!     *pc += 1;
//!     if mem.len() <= *pc {
//!         mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!     }
//!     mem[*pc] = mem[*pc].wrapping_add(2);
//!     *pc += 2;
//!     if mem.len() <= *pc {
//!         mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!     }
//!     mem[*pc] = mem[*pc].wrapping_add(1);
//!     if *pc < 1 {
//!         panic!("BrainFuck Program Counter reduced to below zero !!!");
//!     } else {
//!         *pc -= 1;
//!     }
//!     while mem[*pc] != 0 {
//!         mem[*pc] = mem[*pc].wrapping_sub(1);
//!         while mem[*pc] != 0 {
//!             *pc += 2;
//!             if mem.len() <= *pc {
//!                 mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!             }
//!             mem[*pc] = mem[*pc].wrapping_add(1);
//!             if *pc < 2 {
//!                 panic!("BrainFuck Program Counter reduced to below zero !!!");
//!             } else {
//!                 *pc -= 2;
//!             }
//!             mem[*pc] = mem[*pc].wrapping_sub(1);
//!         }
//!         mem[*pc] = mem[*pc].wrapping_add(1);
//!         *pc += 2;
//!         if mem.len() <= *pc {
//!             mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!         }
//!     }
//!     *pc += 1;
//!     if mem.len() <= *pc {
//!         mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!     }
//!     mem[*pc] = mem[*pc].wrapping_add(1);
//!     while mem[*pc] != 0 {
//!         mem[*pc] = mem[*pc].wrapping_sub(1);
//!         if *pc < 3 {
//!             panic!("BrainFuck Program Counter reduced to below zero !!!");
//!         } else {
//!             *pc -= 3;
//!         }
//!         while mem[*pc] != 0 {
//!             mem[*pc] = mem[*pc].wrapping_sub(1);
//!             *pc += 1;
//!             if mem.len() <= *pc {
//!                 mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!             }
//!             while mem[*pc] != 0 {
//!                 mem[*pc] = mem[*pc].wrapping_add(1);
//!                 while mem[*pc] != 0 {
//!                     mem[*pc] = mem[*pc].wrapping_sub(1);
//!                 }
//!                 mem[*pc] = mem[*pc].wrapping_add(1);
//!                 *pc += 1;
//!                 if mem.len() <= *pc {
//!                     mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!                 }
//!                 mem[*pc] = mem[*pc].wrapping_add(2);
//!                 *pc += 3;
//!                 if mem.len() <= *pc {
//!                     mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!                 }
//!                 mem[*pc] = mem[*pc].wrapping_sub(1);
//!                 if *pc < 2 {
//!                     panic!("BrainFuck Program Counter reduced to below zero !!!");
//!                 } else {
//!                     *pc -= 2;
//!                 }
//!             }
//!             if *pc < 1 {
//!                 panic!("BrainFuck Program Counter reduced to below zero !!!");
//!             } else {
//!                 *pc -= 1;
//!             }
//!             while mem[*pc] != 0 {
//!                 if *pc < 1 {
//!                     panic!("BrainFuck Program Counter reduced to below zero !!!");
//!                 } else {
//!                     *pc -= 1;
//!                 }
//!             }
//!             *pc += 2;
//!             if mem.len() <= *pc {
//!                 mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!             }
//!             mem[*pc] = mem[*pc].wrapping_add(6);
//!             while mem[*pc] != 0 {
//!                 if *pc < 2 {
//!                     panic!("BrainFuck Program Counter reduced to below zero !!!");
//!                 } else {
//!                     *pc -= 2;
//!                 }
//!                 mem[*pc] = mem[*pc].wrapping_add(5);
//!                 *pc += 2;
//!                 if mem.len() <= *pc {
//!                     mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!                 }
//!                 mem[*pc] = mem[*pc].wrapping_sub(1);
//!             }
//!             mem[*pc] = mem[*pc].wrapping_add(1);
//!             if *pc < 2 {
//!                 panic!("BrainFuck Program Counter reduced to below zero !!!");
//!             } else {
//!                 *pc -= 2;
//!             }
//!             mem[*pc] = mem[*pc].wrapping_add(2);
//!             print!("{}", mem[*pc] as char);
//!             while mem[*pc] != 0 {
//!                 mem[*pc] = mem[*pc].wrapping_sub(1);
//!             }
//!             if *pc < 2 {
//!                 panic!("BrainFuck Program Counter reduced to below zero !!!");
//!             } else {
//!                 *pc -= 2;
//!             }
//!         }
//!         *pc += 1;
//!         if mem.len() <= *pc {
//!             mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!         }
//!         print!("{}", mem[*pc] as char);
//!         *pc += 1;
//!         if mem.len() <= *pc {
//!             mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!         }
//!         mem[*pc] = mem[*pc].wrapping_add(1);
//!         while mem[*pc] != 0 {
//!             *pc += 2;
//!             if mem.len() <= *pc {
//!                 mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!             }
//!         }
//!         *pc += 1;
//!         if mem.len() <= *pc {
//!             mem.append(&mut vec![0; *pc - mem.len() + 1]);
//!         }
//!         mem[*pc] = mem[*pc].wrapping_add(1);
//!     }
//! })
//! ```


pub use bflib_proc_macro::brain_fuck;

/// Represents a brainfuck codeblock
/// ```
/// brain_fuck!(
///     ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
///     >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
/// )
/// ```
pub struct BrainfuckBlock {
    code: &'static dyn Fn(&mut usize, &mut Vec<u8>),
    opt_env: Option<(usize, Vec<u8>)>,
    runned: bool
}

impl BrainfuckBlock {
    /// Constructor of `BrainfuckBlock`
    /// 
    /// receives output of proc marco
    /// 
    /// **HUMAN NEVER USE**
    pub fn new(code: &'static dyn Fn(&mut usize, &mut Vec<u8>)) -> BrainfuckBlock {
        BrainfuckBlock { code, opt_env: None, runned: false }
    }
    /// Sets the Program Counter `pc` and Memory `mem` of the Brainfuck codeblock
    /// Returns self
    pub fn env(mut self, pc: usize, mem: Vec<u8>) -> Self {
        self.opt_env = Some((pc, mem));
        self
    }

    fn run(&mut self) {
        let (mut pc, mut mem) = (0, vec![]);
        if self.opt_env.is_some() {
            (pc, mem) = self.opt_env.take().unwrap();
        }
        (self.code)(&mut pc, &mut mem);
        self.opt_env = Some((pc, mem));
    }
}

impl Into<(usize, Vec<u8>)> for BrainfuckBlock {
    /// obtain `(pc: usize, mem: Vec<u8>)` after running
    fn into(mut self) -> (usize, Vec<u8>) {
        self.runned = true;
        self.run();
        self.opt_env.take().unwrap()
    }
}

impl Drop for BrainfuckBlock {
    /// run the code on dropping if the block never runned before via `into`
    fn drop(&mut self) {
        if !self.runned {
            self.run();
        }
    }
}