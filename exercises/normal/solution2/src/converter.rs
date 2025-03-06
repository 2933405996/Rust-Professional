use regex::Regex;

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    let re = Regex::new(r"(\d+)\((\d+)\)").unwrap();

    match re.captures(num_str) {
        Some(caps) => {
            let num_str = caps[1].parse::<u32>().unwrap().to_string();
            let radix = caps[2].parse::<u32>().unwrap();
            let mut num = u32::from_str_radix(&num_str, radix).unwrap();
            let digits = "0123456789abcdef";

            let mut result = String::new();
            while num > 0 {
                let digit = (num % to_base) as usize;
                result.push(digits.chars().nth(digit).unwrap());
                num /= to_base;
            }
            result.chars().rev().collect::<String>()
        }
        None => num_str.to_string(),
    }
}
