use std::collections::HashMap;

fn main() {
    let mut valid_codes = 0;
    for i in 123257..647015 {
        if is_valid(&i.to_string(), false) {
            valid_codes += 1;
        }
    }
    println!("{}", valid_codes);

    valid_codes = 0;
    for i in 123257..647015 {
        if is_valid(&i.to_string(), true) {
            valid_codes += 1;
        }
    }
    println!("{}", valid_codes);
}

fn is_valid(code: &str, strict: bool) -> bool {
    let mut has_double: bool = false;
    let mut chars = code.chars();
    let mut chars_map: HashMap<Option<u32>, i8> = HashMap::new();
    let mut val = match chars.next() {
        Some(x) => x,
        None => panic!("must pass a string !"),
    };
    chars_map.insert(val.to_digit(10), 1);

    loop {
        match chars.next() {
            Some(x) => {
                let prev = val.to_digit(10);
                let actual = x.to_digit(10);
                if prev > actual {
                    return false;
                }

                if prev == actual {
                    has_double = true;
                }

                let times: i8;
                match chars_map.get(&actual) {
                    Some(&number) => times = number + 1,
                    _ => times = 1,
                }
                chars_map.insert(actual, times);

                val = x;
            }
            None => break,
        }
    }

    if !strict {
        return has_double;
    }

    chars_map.values().any(|v| v == &2)
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn it_should_know_if_a_code_is_valid() {
        assert_eq!(is_valid(&"111111", false), true);
        assert_eq!(is_valid(&"223450", false), false);
        assert_eq!(is_valid(&"123789", false), false);
    }

    #[test]
    fn it_should_know_if_a_code_is_strictly_valid() {
        assert_eq!(is_valid(&"112233", true), true);
        assert_eq!(is_valid(&"123444", true), false);
        assert_eq!(is_valid(&"111122", true), true);
    }
}
