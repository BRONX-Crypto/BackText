use bitvec::prelude::*;
use std::io::Write;
use std::path::Path;
pub fn Lex(source: &str) -> BitVec<u8, Msb0> {
    let ch: Vec<char> = source.chars().collect();
    let mut bits: BitVec<u8, Msb0> = BitVec::new();
    let mut i = 0;
    while i < ch.len() {
        if ch[i].is_ascii_alphabetic() {
            let mut start = i;
            while i < ch.len() && ch[i].is_ascii_alphabetic() {
                i += 1;
            }
            let data: String = ch[start..i].iter().collect();
            if data == "SetLen" {
                bits.push(true); bits.push(false); bits.push(true); bits.push(false); bits.push(false);
                while i < ch.len() && ch[i] != ')' {
                        match ch[i] {
                        '0' => bits.push(false),
                        '1' => bits.push(true),
                        ')' => (),
                        '(' => (),
                        _ => (),
                    }
                    i += 1;
                }
                if ch[i] == ')' {
                    i += 1;
                }
            }
            if data == "push" {
                bits.push(false); bits.push(false); bits.push(false); bits.push(false); bits.push(true);
                while i < ch.len() && ch[i] != ')' {
                    match ch[i] {
                        '0' => bits.push(false),
                        '1' => bits.push(true),
                        _ => println!("find not allowed character: {}", ch[i]),
                    }
                    i += 1;
                }
                if ch[i] == ')' {
                    i += 1;
                }
            }
            if data == "pop" {
                bits.push(false); bits.push(false); bits.push(false); bits.push(true); bits.push(false);
            }
            if data == "nop" {
                bits.push(false); bits.push(false); bits.push(false); bits.push(false); bits.push(false);
            }
            if data == "plus" {
                bits.push(false); bits.push(false); bits.push(false); bits.push(true); bits.push(true);
            }
            if data == "minus" {
                bits.push(false); bits.push(false); bits.push(true); bits.push(false); bits.push(false);
            }

            continue;

    }
    if ch[i].is_whitespace() {
        i += 1;
        continue;
    }
    if ch[i] == ';' {
        i += 1;
        while i < ch.len() && ch[i] != ';' {
            i += 1;
        }
        continue;
    }
    }
    bits
}
fn main() {
    let name = "BackText";
    let vc = vec![0, 1, 0];
println!("{} {} ({}.{}.{})", name, vc[0], vc[0], vc[1], vc[2]);
        print!("Please Enter BackText File To Create Binary Backpack Format");
        std::io::stdout().flush().unwrap();
        let mut i = String::new();
        std::io::stdin().read_line(&mut i).unwrap();
        let i = i.trim();
        let pointer = &i;
        let path = Path::new(pointer);
        let F = std::fs::read_to_string(path).unwrap();
        if path.exists() {
            let data = Lex(&F);
            let bytes = data.into_vec();
            let fonrn = path.file_name().unwrap().to_string_lossy().to_string();
            let form = format!("{}.backpack", fonrn);
            let mut cf = std::fs ::File::create(&form).unwrap();
            cf.write_all(&bytes).unwrap();
            println!("Successfuly Generated {} From {}", form, fonrn);
        }
        if !path.exists() {
            println!("This Path or File not exists: {:?}", path);
        }
}
