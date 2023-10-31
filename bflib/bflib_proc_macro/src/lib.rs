use std::str::FromStr;
use proc_macro::TokenStream;

fn transpiler(input: String) -> String {
    let mut res = String::from(r##"
        ::bflib::BrainfuckBlock::new(&|pc: &mut usize, mem: &mut Vec<u8>| {
            while mem.len() <= *pc {
                mem.push(0);
            }
    "##);
    for c in input.chars() {
        match c {
            '>' => {
                res.push_str(r##"
                *pc += 1;
                if mem.len() == *pc {
                    mem.push(0);
                }
                "##);
            },
            '<' => {
                res.push_str("*pc -= 1;");
            },
            '+' => {
                res.push_str("mem[*pc] = mem[*pc].wrapping_add(1);");
            },
            '-' => {
                res.push_str("mem[*pc] = mem[*pc].wrapping_sub(1);");
            },
            '.' => {
                res.push_str("print!(\"{}\", mem[*pc] as char);");
            },
            ',' => {
                res.push_str(r##"
                {
                    let mut buffer = [0; 1];
                    std::io::Read::read_exact(&mut std::io::stdin(), &mut buffer).unwrap();
                    mem[*pc] = buffer[0];
                }
                "##);
            },
            '[' => {
                res.push_str("while mem[*pc] != 0 {");
            },
            ']' => {
                res.push_str("}");
            },
            _ => {}
        }
    }
    res.push_str("})");
    res
}


/// inline Brainfuck code  
/// 
/// Example of Hello World:
/// ```
/// let (pc: usize, mem: Vec<u8>) = brain_fuck!(
///     ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
///     >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
/// );
/// println!("{:?}", (pc, mem));
/// ```
#[proc_macro]
pub fn brain_fuck(_item: TokenStream) -> TokenStream {
    let input = _item.to_string();
    TokenStream::from_str(&transpiler(input)).unwrap()
}