use std::fs;

fn main() {
    let file_path = "1.input";

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let result: i32 = input.split_terminator("\n")
        .map(|x| -> i32 {
            let nums: Vec<&str> = x.matches(char::is_numeric).collect();

            nums.iter().nth(0).unwrap().parse::<i32>().unwrap() * 10 +
                nums.iter().last().unwrap().parse::<i32>().unwrap()
        })
        .sum();

    println!("{}", result);
}
