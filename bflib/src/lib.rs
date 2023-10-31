pub use bflib_proc_macro::brain_fuck;

pub struct BrainfuckBlock {
    code: &'static dyn Fn(&mut usize, &mut Vec<u8>),
    runned: bool
}

impl BrainfuckBlock {
    pub fn new(code: &'static dyn Fn(&mut usize, &mut Vec<u8>)) -> BrainfuckBlock {
        BrainfuckBlock { code, runned: false }
    }
    pub fn env(mut self, mut pc: usize, mut mem: Vec<u8>) -> (usize, Vec<u8>) {
        (self.code)(&mut pc, &mut mem);
        self.runned = true;
        (pc, mem)
    }
}

impl Into<(usize, Vec<u8>)> for BrainfuckBlock {
    fn into(mut self) -> (usize, Vec<u8>) {
        self.runned = true;
        let mut pc = 0;
        let mut mem = vec![];
        (self.code)(&mut pc, &mut mem);
        (pc, mem)
    }
}

impl Drop for BrainfuckBlock {
    fn drop(&mut self) {
        if !self.runned {
            (self.code)(&mut 0, &mut vec![])
        }
    }
}