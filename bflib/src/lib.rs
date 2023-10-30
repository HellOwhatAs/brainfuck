use std::str::FromStr;
use proc_macro::TokenStream;

const CELL_SIZE: usize = 16;
fn parse(code: &[u8], skip: bool, cells: &mut [u8; CELL_SIZE],
    pc: &mut usize, output: &mut Vec<u8>) -> usize {
    let mut idx = 0;
    while idx < code.len() {
        let c = code[idx];
        match c {
            b'+' if !skip => cells[*pc] += 1,
            b'-' if !skip => cells[*pc] -= 1,
            b'.' if !skip => output.push(cells[*pc]),
            b'>' if !skip => *pc += 1,
            b'<' if !skip => *pc -= 1,
            b'[' => {
                while !skip && cells[*pc] != 0 {
                    parse(&code[idx+1..], false, cells, pc, output);
                }
                idx += parse(&code[idx+1..], true, cells, pc, output) + 1;
            },
            b']' => return idx,
            _ => {}
        }
        idx += 1;
    }
    idx
}

#[proc_macro]
pub fn brain_fuck(_item: TokenStream) -> TokenStream {
    let input = _item.to_string();
    let mut cells: [u8; CELL_SIZE] = [0; CELL_SIZE];
    let mut pc = 0;
    let mut output = Vec::<u8>::new();

    parse(&input.as_bytes(), false, &mut cells, &mut pc, &mut output);

    TokenStream::from_str(
        &format!("\"{}\"", std::str::from_utf8(&output).unwrap())
    ).unwrap()
}