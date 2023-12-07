use std::fs::File;
use std::io::Read;

pub fn day3_part1() {
    //Take input from a text File
    let mut file = File::open("day3.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split("\n").collect();

    //Create a variable to store the output
    let mut out = 0;

    let mut pos_y = 0;
    for row in &input  {

        let mut number = String::new();
        let mut pos_x = 0;
        //find the numbers
        for c in row.chars() {
            if c.is_numeric() {
                number += &c.to_string();
            }
            if !c.is_numeric() || pos_x == row.len()-1 {
                if number != "" {
                    let start_value = if pos_x - number.len() == 0 {0} else if c.is_numeric(){pos_x - number.len()} else {pos_x - number.len() - 1};
                    let end_value = if pos_x + 1 > row.len() {pos_x} else {pos_x + 1};

                    let proceeding_symbol = pos_x - number.len() != 0 && input[pos_y].chars().nth(start_value).unwrap() != '.';
                    let mut above_symbol = false;
                    let mut below_symbol = false;

                    for i in start_value .. end_value {
                        if pos_y != 0{
                            let above = input[pos_y-1].chars().nth(i).unwrap();
                            //can't be a . or a number
                            if above != '.' && !above.is_numeric() {
                                above_symbol = true;
                            }
                        }
                        if pos_y != input.len()-1 {
                            let below = input[pos_y+1].chars().nth(i).unwrap();
                            //can't be a . or a number
                            if below != '.' && !below.is_numeric() {
                                below_symbol = true;
                            }
                        }
                    }
                    if c != '.' && !c.is_numeric() || proceeding_symbol || above_symbol || below_symbol {
                        out += number.parse::<i32>().unwrap();
                    }
                    number.clear();
                }
            }
            pos_x += 1;
        }
        pos_y += 1;
    }


    //Print the output
    println!("Day 3 / Part 1 Output is: {}", out);

}

pub fn day3_part2() {
    //Take input from a text File
    let mut file = File::open("day3.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split("\n").collect();

    //Create a variable to store the output
    let mut out = 0;

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for pos_y in 0..input.len() as i32{
        let mut pos_x : i32 = 0;
        let mut number = String::new();
        for c in input[pos_y as usize].chars() {
            pos_x += 1;
            if c == '*' {
                //store the symbol and coordinates
                symbols.push((c, pos_x, pos_y));
            }else if c.is_numeric() {
                number += &c.to_string();
            }
            if(!c.is_numeric() || pos_x == input[pos_y as usize].len() as i32)  && number != "" {
                numbers.push((number.parse::<i32>().unwrap(), pos_x-number.len() as i32 - 1 .. pos_x + 1, pos_y-1..pos_y+2));
                number.clear();
            }
        }
    }

    //Loop through the symbols
    for symbol in &symbols{
        let mut matching_numbers = Vec::new();
        for number in &numbers{
            if number.1.contains(&symbol.1) && number.2.contains(&symbol.2){
                matching_numbers.push(number.0);
            }
        }
        if matching_numbers.len() == 2 {
            out += matching_numbers[0] * matching_numbers[1];
        }
    }

    //Print the output
    println!("Day 3 / Part 2 Output is: {}", out);

}