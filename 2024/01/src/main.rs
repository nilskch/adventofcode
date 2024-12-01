use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let input_path = "./src/input.txt";
    let input_file = File::open(input_path)?;
    let input_reader = io::BufReader::new(input_file);

    let mut left_numbers = vec![];
    let mut right_numbers = vec![];

    for line in input_reader.lines() {
        let line = line?;
        let numbers = line
            .split_whitespace()
            .map(|num| num.parse::<u64>().expect("invalid number"))
            .collect::<Vec<_>>();

        if let [left, right] = numbers[..] {
            left_numbers.push(left);
            right_numbers.push(right);
        } else {
            panic!("if you read this in the console, you're a shit coder");
        }
    }

    left_numbers.sort();
    right_numbers.sort();

    let mut result_1 = 0;

    for (i, left_num) in left_numbers.iter().enumerate() {
        let right_num = right_numbers.get(i).expect("this index must exist");
        result_1 += left_num.abs_diff(*right_num);
    }

    // part 2:
    let mut right_counter: HashMap<u64, u64> = Default::default();
    for num in right_numbers {
        right_counter
            .entry(num)
            .and_modify(|num| *num += 1)
            .or_insert(1);
    }

    let mut result_2 = 0;

    for num in left_numbers {
        if let Some(counter) = right_counter.get(&num) {
            result_2 += num * counter;
        }
    }

    println!("result part 1: {}", result_1);
    println!("result part 2: {}", result_2);

    Ok(())
}
