use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: day01b <input file name>");
        process::exit(1);
    }

    let input_file_name = &args[1];
    let input_file_contents = fs::read_to_string(input_file_name)
        .expect(format!("Failed to read {}", input_file_name).as_str());

    let mut sum = 0;

    for line in input_file_contents.lines() {
        let mut first_num: i8 = -1;
        let mut last_num: i8 = -1;

        for (i, char) in line.chars().enumerate() {
            let num: i8;

            if char.is_numeric() {
                num = char.to_digit(10).unwrap() as i8;
            } else {
                num = match &line[i..] {
                    s if s.starts_with("one") => 1,
                    s if s.starts_with("two") => 2,
                    s if s.starts_with("three") => 3,
                    s if s.starts_with("four") => 4,
                    s if s.starts_with("five") => 5,
                    s if s.starts_with("six") => 6,
                    s if s.starts_with("seven") => 7,
                    s if s.starts_with("eight") => 8,
                    s if s.starts_with("nine") => 9,
                    _ => -1,
                };
            }

            if num != -1 {
                if first_num == -1 {
                    first_num = num;
                }
                last_num = num;
            }
        }

        sum += format!("{}{}", first_num, last_num).parse::<i32>().unwrap();
    }

    println!("Sum: {}", sum);
}
