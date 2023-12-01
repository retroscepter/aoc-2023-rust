use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: day01a <input file name>");
        process::exit(1);
    }

    let input_file_name = &args[1];
    let input_file_contents = fs::read_to_string(input_file_name)
        .expect(format!("Failed to read {}", input_file_name).as_str());

    let mut sum = 0;

    for line in input_file_contents.lines() {
        let mut first_num: i8 = -1;
        let mut last_num: i8 = -1;

        for c in line.chars() {
            if c.is_numeric() {
                let num = c.to_digit(10).unwrap() as i8;

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
