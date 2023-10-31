//! # BrainFuck compiler using Rust proc macro
//! More precisely, the BrainFuck-to-Rust transpiler using Rust proc macro
//! 
//! ## Examples:
//! 
//! 1. Runs on `into` called
//!    (using `into` method to obtain `(pc: usize, mem: Vec<u8>)`)
//!    ```rust
//!    let (pc, mem) = brain_fuck!(
//!        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
//!        >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
//!    ).into();
//!    println!("{:?}", (pc, mem));
//!    ```
//! 2. Runs on `env` called
//!    (`env` method sets `pc` and `mem` for the block)
//!    (`env` method also returns `(pc: usize, mem: Vec<u8>)`)
//!    ```rust
//!    let mut pc = 0;
//!    let mut mem = vec![72, 101, 108, 108, 79, 119, 104, 97, 116, 65, 115, 10, 0];
//!    let (pc, mem) = brain_fuck!(
//!        [.>]
//!    ).env(pc, mem);
//!    println!("{:?}", (pc, mem));
//!    ```
//! 3. Runs on drop
//!    ```rust
//!    brain_fuck!(
//!        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
//!        >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
//!    );
//!    ```
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
//! });
//! ```


pub use bflib_proc_macro::brain_fuck;

/// Represents a brainfuck codeblock
/// ```
/// brain_fuck!(
///     ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
///     >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
/// )
/// ```
/// The brainfuck code in it runs on
/// 1. drop of the block
///    ```
///    brain_fuck!(
///        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
///        >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
///    );
///    ```
/// 2. calling `env`
///    ```
///    let (pc, mem) = brain_fuck!(
///        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
///        >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
///    ).env(pc, mem);
///    ```
/// 3. calling `into`
///    ```
///    let (pc, mem) = brain_fuck!(
///        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
///        >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
///    ).into();
///    ```
/// **But notice that THIS WON'T RUN**
/// ```
/// let tmp: BrainfuckBlock = brain_fuck!(
///     ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
///     >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
/// );
/// ```
pub struct BrainfuckBlock {
    code: &'static dyn Fn(&mut usize, &mut Vec<u8>),
    runned: bool
}

impl BrainfuckBlock {
    /// Constructor of `BrainfuckBlock`
    /// 
    /// receives output of proc marco
    /// 
    /// **HUMAN NEVER USE**
    pub fn new(code: &'static dyn Fn(&mut usize, &mut Vec<u8>)) -> BrainfuckBlock {
        BrainfuckBlock { code, runned: false }
    }
    /// Sets the Program Counter `pc` and Memory `mem` of the Brainfuck codeblock
    /// and runs the code
    /// 
    /// Returns `(pc: usize, mem: Vec<u8>)` after running
    pub fn env(mut self, mut pc: usize, mut mem: Vec<u8>) -> (usize, Vec<u8>) {
        (self.code)(&mut pc, &mut mem);
        self.runned = true;
        (pc, mem)
    }
}

impl Into<(usize, Vec<u8>)> for BrainfuckBlock {
    /// obtain `(pc: usize, mem: Vec<u8>)` after running
    fn into(mut self) -> (usize, Vec<u8>) {
        self.runned = true;
        let mut pc = 0;
        let mut mem = vec![];
        (self.code)(&mut pc, &mut mem);
        (pc, mem)
    }
}

impl Drop for BrainfuckBlock {
    /// runs the code on dropping if the block never runned
    /// 
    /// Case:
    /// ```
    /// brain_fuck!(
    ///     ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
    ///     >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    /// );
    /// ```
    fn drop(&mut self) {
        if !self.runned {
            (self.code)(&mut 0, &mut vec![])
        }
    }
}