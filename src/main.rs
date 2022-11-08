use std::env::args;

fn main() {
    let input = args().skip(1).collect();
    let output = alternate_case(input);
    println!("{output}");
}

// Function that takes a string and returns a string.
// The returning string is alternates with lower and upper case
// and begins with a upper case.
fn alternate_case(s: String) -> String {
    let mut result = String::new();
    let mut upper = true;
    for c in s.chars() {
        if !c.is_alphabetic() {
            result.push(c);
            continue;
        }

        if upper {
            result.push(c.to_uppercase().next().unwrap());
        } else {
            result.push(c.to_lowercase().next().unwrap());
        }
        upper = !upper;
    }
    result
}


mod tests {
    #[test]
    fn test_alternate_case() {
        assert_eq!("HeLlO, wOrLd!", super::alternate_case("Hello, world!".to_string()));
    }
}