use std::collections::HashMap;

pub fn instruction_a(line: &str, var_history: &mut HashMap<String, u16>) -> String {
    let variable = line.replace("@", "");
    let value: Result<u16, _> = variable.to_string().parse();
    match value {
        Ok(num) => {
            // If value is a number
            let mut b_num = format!("{:015b}", num);
            b_num = "0".to_string() + b_num.as_str();
            b_num
        }
        _ => {
            let mut b_num = format!(
                "{:015b}",
                var_history
                    .get_key_value(variable.as_str())
                    .expect("Variable name not found on symbols table")
                    .1
            );
            b_num = "0".to_string() + b_num.as_str();
            b_num
        }
    }
}
