mod instruction_a;
mod instruction_c;

use core::panic;
use std::collections;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("In file {} \n", path);

    let start = Instant::now();

    let contents = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            panic!("{}", e);
        }
    };

    // println!("\n{}", contents); // all file content

    let contents_vector: Vec<&str> = contents.split('\n').collect();
    let copy_contens_vector = contents_vector.clone();
    let mut s_table = symbol_table(copy_contens_vector);
    // print!("{:#?}", s_table);
    let decode_asm = decode(contents_vector, &mut s_table);

    let (sub_path, file_name_extension) = path
        .rsplit_once('/')
        .expect("Error on declaring destine path for the compiled file");
    let (file_name, _extension) = file_name_extension.rsplit_once('.').expect("error");

    let dest_path = sub_path.to_string() + "/RustCompile/" + file_name + ".hack";
    let w_file = fs::write(dest_path, decode_asm.join("\n"));
    match w_file {
        Ok(()) => println!("\nWritting Succesfull"),
        Err(error) => println!("\nWritting Unsuccesfull {}", error),
    };

    let finish = Instant::now();
    println!("{:?}", finish.duration_since(start).as_micros());
}

fn symbol_table(vec: Vec<&str>) -> HashMap<String, u16> {
    let mut var_history: HashMap<String, u16> = collections::HashMap::from([
        ("R0".to_string(), 0),
        ("R1".to_string(), 1),
        ("R2".to_string(), 2),
        ("R3".to_string(), 3),
        ("R4".to_string(), 4),
        ("R5".to_string(), 5),
        ("R6".to_string(), 6),
        ("R7".to_string(), 7),
        ("R8".to_string(), 8),
        ("R9".to_string(), 9),
        ("R10".to_string(), 10),
        ("R11".to_string(), 11),
        ("R12".to_string(), 12),
        ("R13".to_string(), 13),
        ("R14".to_string(), 14),
        ("R15".to_string(), 15),
        ("SCREEN".to_string(), 16384),
        ("KBD".to_string(), 24572),
        ("SP".to_string(), 0),
        ("LCL".to_string(), 1),
        ("ARG".to_string(), 2),
        ("THIS".to_string(), 3),
        ("THAT".to_string(), 4),
    ]);
    let mut var_pos: u16 = 16;
    let mut line_iter = 0;
    for line in vec {
        let inst = line.trim();
        match inst.find('@') {
            Some(0) => {
                let var = inst.replace("@", "");
                match var_history.contains_key(var.as_str()) {
                    true => {}
                    false => {
                        let try_num: Result<u16, _> = var.parse();
                        match try_num {
                            Ok(_) => {}
                            _ => {
                                var_history.insert(var, var_pos);
                                var_pos += 1;
                            }
                        }
                    }
                }
            }
            _ => match inst.find('(') {
                Some(0) => {
                    let tag = inst.replace("(", "").replace(")", "");
                    match var_history.contains_key(tag.as_str()) {
                        true => {}
                        false => {
                            // println!("Tag: {} Line_iter: {}", tag, line_iter);
                            var_history.insert(tag, line_iter);
                        }
                    }
                }
                _ => {}
            },
        };
        // println!("{}, iter {}", line, line_iter);
        match line.find('/').or(line.find('(')) {
            Some(0) => {
                continue;
            }
            _ => {
                line_iter += 1;
            }
        }
        // println!("Line_iter {}", line_iter);
    }
    var_history
}

fn decode(vec: Vec<&str>, var_history: &mut HashMap<String, u16>) -> Vec<String> {
    let mut contents: Vec<String> = Vec::new();
    let comp_decode_table = instruction_c::comp_decode_table();
    let dest_decode_table = instruction_c::dest_decode_table();
    let jmp_decode_table = instruction_c::jmp_decode_table();
    for mut full_line in vec {
        if full_line.is_empty() {
            break;
        }
        if full_line.trim().find('/') == Some(0) || full_line.trim().find('(') == Some(0) {
            // Maneja comentarios y los tags, y se los salta
            continue;
        }
        match full_line.split_once('/') {
            // Inline comment
            Some((inst, _comment)) => {
                full_line = inst;
            }
            _ => {}
        };
        match full_line.trim().find('@') {
            Some(0) => {
                let ins_a = instruction_a::instruction_a(full_line.trim(), var_history);
                contents.push(ins_a);
            }
            _ => {
                let ins_c = instruction_c::instruction_c(
                    full_line.trim(),
                    &comp_decode_table,
                    &dest_decode_table,
                    &jmp_decode_table,
                );
                contents.push(ins_c);
            }
        };
    }
    // println!("CONTENTS VARIABLES:\n{}", var_history_name.join("\n"));
    // println!("CONTENTS:\n \n{}", contents.join("\n"));
    contents
}
