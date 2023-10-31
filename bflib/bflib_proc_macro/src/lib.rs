use std::str::FromStr;
use proc_macro::TokenStream;

fn transpiler(input: String) -> String {
    let mut res = String::from(r##"
        ::bflib::BrainfuckBlock::new(&|pc: &mut usize, mem: &mut Vec<u8>| {
            if mem.len() <= *pc {
                mem.append(&mut vec![0; *pc - mem.len() + 1]);
            }
    "##);
    let mut cache = (' ', 0);
    for c in input.chars().filter(|c| "><+-,.[]".contains(*c)) {
        if c == cache.0 {
            cache.1 += 1;
            continue;
        }
        {
            match cache.0 {
                '>' => {
                    res.push_str(&format!("*pc += {};", cache.1));
                    res.push_str(r##"
                        if mem.len() <= *pc {
                            mem.append(&mut vec![0; *pc - mem.len() + 1]);
                        }
                    "##);
                }
                '<' => {
                    res.push_str(&format!("if *pc < {0} {{ panic!(\"BrainFuck Program Counter reduced to below zero !!!\"); }} else {{ *pc -= {0}; }}", cache.1));
                }
                '+' => {
                    res.push_str(&format!("mem[*pc] = mem[*pc].wrapping_add({});", cache.1));
                }
                '-' => {
                    res.push_str(&format!("mem[*pc] = mem[*pc].wrapping_sub({});", cache.1));
                }
                _ => {}
            }
            cache = (' ', 0);
        }
        match c {
            '>' | '<' | '+' | '-' => {
                cache = (c, 1);
            }
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
    {
        match cache.0 {
            '>' => {
                res.push_str(&format!("*pc += {};", cache.1));
                res.push_str(r##"
                    if mem.len() <= *pc {
                        mem.append(&mut vec![0; *pc - mem.len() + 1]);
                    }
                "##);
            }
            '<' => {
                res.push_str(&format!("if *pc < {0} {{ panic!(\"BrainFuck Program Counter reduced to below zero !!!\"); }} else {{ *pc -= {0}; }}", cache.1));
            }
            '+' => {
                res.push_str(&format!("mem[*pc] = mem[*pc].wrapping_add({});", cache.1));
            }
            '-' => {
                res.push_str(&format!("mem[*pc] = mem[*pc].wrapping_sub({});", cache.1));
            }
            _ => {}
        }
    }
    res.push_str("})");
    res
}


/// inline Brainfuck code  
/// 
/// Examples:
/// 
/// 1. Hello World
///    (run on dropping)
///    ```rust
///    brain_fuck!(
///        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
///        >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
///    );
///    ```
/// 2. using `into` method to obtain `(pc: usize, mem: Vec<u8>)` after running
///    (run on `into` calling)
///    ```rust
///    let (pc, mem) = brain_fuck!(
///        ++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[
///            -<<<[
///                ->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<
///            ]>.>+[>>]>+
///        ]
///    ).into();
///    println!("{:?}", (pc, mem));
///    ```
/// 3. use `env` method to set _Program Counter_ `pc` and _Memory_ `mem` for brainfuck codeblock
///    (run on dropping)
///    ```rust
///    brain_fuck!(
///        [.>]
///    ).env(0, vec![79, 75, 10]);
///    ```
/// 4. Altogether
///    (run on `into` calling)
///    ```rust
///    let (pc, mem) = brain_fuck!(
///        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
///        >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
///    ).env(0, vec![]).into();
///    println!("{:?}", (pc, mem));
///    ```
#[proc_macro]
pub fn brain_fuck(_item: TokenStream) -> TokenStream {
    let input = _item.to_string();
    TokenStream::from_str(&transpiler(input)).unwrap()
}