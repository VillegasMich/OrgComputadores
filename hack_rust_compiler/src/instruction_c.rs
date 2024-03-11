use std::collections::HashMap;

pub fn instruction_c(
    instruction: &str,
    comp_decode_table: &HashMap<String, String>,
    dest_decode_table: &HashMap<String, String>,
    jmp_decode_table: &HashMap<String, String>,
) -> String {
    // dest=comp;jmp
    // dest and jmp may be empty
    let mut b_num = "111".to_string();
    let mut b_dest = "".to_string();
    let mut comp_with_jmp = "";
    let mut comp_no_jmp = "";
    let mut b_jmp = "".to_string();
    match instruction.find('=') {
        Some(_) => {
            // Has dest part
            let ins_eq_split = instruction
                .split_once('=')
                .expect("Error with '=' isntruction");
            let dest = ins_eq_split.0;
            comp_with_jmp = ins_eq_split.1;
            comp_no_jmp = ins_eq_split.1;
            b_dest = dest_decode_table
                .get_key_value(dest)
                .expect("Error con getting the b_dest")
                .1
                .to_owned();
            b_jmp = "000".to_string();
            // println!("b_comp = {} b_dest = {}", b_comp, b_dest);
        }
        None => {}
    };
    match instruction.find(';') {
        Some(_) => {
            // Has jmp part
            let ins_eq_split = instruction
                .split_once(';')
                .expect("Error with ';' isntruction");
            comp_no_jmp = ins_eq_split.0;
            let jmp = ins_eq_split.1;
            b_jmp = jmp_decode_table
                .get_key_value(jmp)
                .expect("Error con getting the b_jmp")
                .1
                .to_owned();
            // println!("b_comp = {} b_jmp = {}", b_comp, b_jmp);
            if b_dest.is_empty() {
                b_dest = "000".to_string();
            }
        }
        _ => {}
    };
    match comp_with_jmp.split_once(';') {
        Some((comp, _)) => {
            // println!("comp = {}", comp);
            let b_comp = comp_decode_table
                .get_key_value(comp)
                .expect("Error con getting the b_comp")
                .1
                .to_owned();
            b_num = b_num + b_comp.as_str() + b_dest.as_str() + b_jmp.as_str();
            b_num
        }
        _ => {
            // println!("instruction = {}, comp = {}", instruction, comp_no_jmp);
            let b_comp = comp_decode_table
                .get_key_value(comp_no_jmp)
                .expect("Error con getting the b_comp")
                .1
                .to_owned();
            b_num = b_num + b_comp.as_str() + b_dest.as_str() + b_jmp.as_str();
            b_num
        }
    }
}

pub fn comp_decode_table() -> HashMap<String, String> {
    let table = HashMap::from([
        ("0".to_string(), "0101010".to_string()),
        ("1".to_string(), "0111111".to_string()),
        ("-1".to_string(), "0111010".to_string()),
        ("D".to_string(), "0001100".to_string()),
        ("A".to_string(), "0110000".to_string()),
        ("!D".to_string(), "0001101".to_string()),
        ("!A".to_string(), "0110001".to_string()),
        ("-D".to_string(), "0001111".to_string()),
        ("-A".to_string(), "0110001".to_string()),
        ("D+1".to_string(), "0011111".to_string()),
        ("A+1".to_string(), "0110111".to_string()),
        ("D-1".to_string(), "0001110".to_string()),
        ("A-1".to_string(), "0110010".to_string()),
        ("D+A".to_string(), "0000010".to_string()),
        ("D-A".to_string(), "0010011".to_string()),
        ("A-D".to_string(), "0000111".to_string()),
        ("D&A".to_string(), "0000000".to_string()),
        ("D|A".to_string(), "0010101".to_string()),
        ("M".to_string(), "1110000".to_string()),
        ("!M".to_string(), "1110001".to_string()),
        ("-M".to_string(), "1110011".to_string()),
        ("M+1".to_string(), "1110111".to_string()),
        ("M-1".to_string(), "1110010".to_string()),
        ("D+M".to_string(), "1000010".to_string()),
        ("D-M".to_string(), "1010011".to_string()),
        ("M-D".to_string(), "1000111".to_string()),
        ("D&M".to_string(), "1000000".to_string()),
        ("D|M".to_string(), "1010101".to_string()),
    ]);
    table
}

pub fn dest_decode_table() -> HashMap<String, String> {
    let table = HashMap::from([
        ("".to_string(), "000".to_string()),
        ("M".to_string(), "001".to_string()),
        ("D".to_string(), "010".to_string()),
        ("MD".to_string(), "011".to_string()),
        ("A".to_string(), "100".to_string()),
        ("AM".to_string(), "101".to_string()),
        ("AD".to_string(), "110".to_string()),
        ("AMD".to_string(), "111".to_string()),
    ]);
    table
}

pub fn jmp_decode_table() -> HashMap<String, String> {
    let table = HashMap::from([
        ("".to_string(), "000".to_string()),
        ("JGT".to_string(), "001".to_string()),
        ("JEQ".to_string(), "010".to_string()),
        ("JGE".to_string(), "011".to_string()),
        ("JLT".to_string(), "100".to_string()),
        ("JNE".to_string(), "101".to_string()),
        ("JLE".to_string(), "110".to_string()),
        ("JMP".to_string(), "111".to_string()),
    ]);
    table
}
