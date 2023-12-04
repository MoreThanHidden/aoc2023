use std::fs::File;
use std::io::Read;

pub fn day1_part1() {
    //Take input from a text File
    let mut file = File::open("day1.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    //Split the input into a vector of strings
    let input: Vec<&str> = input.split("\n").collect();

    //Create a variable to store the output
    let mut output = 0;

    //Loop through the rows
    for i in 0..input.len() {
        //Return the digits
        let mut temp = String::new();
        for c in input[i].chars() {
            if c.is_digit(10) {
                temp += &c.to_string();
            }
        }
        //Only add the first and last integer to the output
        temp = temp.chars().take(1).collect::<String>() + &temp.chars().rev().take(1).collect::<String>();
        if(temp.len() < 1) {
            temp += "0";
        }

        output += temp.parse::<i32>().unwrap();
    }

    //Print the output
    println!("Day 1 / Part 1 Output is: {}", output);

}

pub fn day1_part2() {
    //Take input from a text File
    let mut file = File::open("day1.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    //Split the input into a vector of strings
    let input: Vec<&str> = input.split("\n").collect();

    //Create a variable to store the output
    let mut output = 0;

    //Loop through the rows
    for i in 0..input.len() {
        //Get Any Digits
        let mut temp_input = input[i].to_string();

        let digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        let mut word = String::new();

        let mut first = String::new();
        let mut last = String::new();

        //get the first digit
        'outer: for c in temp_input.chars() {
            if c.is_alphabetic() {
                word.push(c);
                for i in 0..digits.len() {
                    if word.contains(&digits[i]) {
                        first = (i + 1).to_string();
                        word.clear();
                        break 'outer;
                    }
                }
            }else{
                first = c.to_string();
                word.clear();
                break;
            }
        }

        //get the last digit
        'outer: for c in temp_input.chars().rev() {
            if c.is_alphabetic() {
                word.push(c);
                for i in 0..digits.len() {
                    if word.contains(&digits[i].chars().rev().collect::<String>()) {
                        last = (i + 1).to_string();
                        word.clear();
                        break 'outer;
                    }
                }
            }else{
                last = c.to_string();
                word.clear();
                break;
            }
        }

        //Only add the first and last integer to the output
        let out = first + &last;
        output += out.parse::<i32>().unwrap();
    }

    //Print the output
    println!("Day 1 / Part 2 Output is: {}", output);
}
