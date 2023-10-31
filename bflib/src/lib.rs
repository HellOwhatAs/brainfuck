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