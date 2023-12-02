use regex::Regex;
use std::fs;

fn get_number(value: &str) -> i32 {
    match value {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => value.parse::<i32>().unwrap(),
    }
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let file_path = "1.input";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let number_regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    // "8twoneh" contains 8, two and one. A normal regex only finds 8 and "two", but not "one".
    // as there is no regex from back, the string is reversed and a reverse regex is used
    let number_regex_reverse = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9])").unwrap();

    let result: i32 = input
        .split_terminator("\n")
        .map(|line| -> i32 {
            let result = get_number(&number_regex.captures_iter(line).nth(0).unwrap()[0]) * 10
                + get_number(
                    &reverse(
                        &number_regex_reverse.captures_iter(
                            &reverse(line)
                        ).nth(0).unwrap()[0]
                    )
                );

            println!("line: {}, reverse: {}, number: {}", line, reverse(line), result);

            return result;
        })
        .sum();

    println!("Result: {}", result);
}
