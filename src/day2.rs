use std::fs::File;
use std::io::Read;

pub fn day2_part1() {
    //Take input from a text File
    let mut file = File::open("day2.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut input: Vec<&str> = input.split("\n").collect();

    //Create a variable to store the output
    let mut output = 0;

    //Loop through the rows
    for i in 0..input.len() {
        //Split by draw
        let draws = input[i].split(": ").collect::<Vec<&str>>()[1].split("; ").collect::<Vec<&str>>();
        let mut possible = true;
        for draw in &draws{
            //Split by colour
            let colours = draw.split(", ").collect::<Vec<&str>>();
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for colour in &colours{
                //Split by number
                let number = colour.split(" ").collect::<Vec<&str>>();
                if number[1] == "red" {red = number[0].parse::<i32>().unwrap();}
                if number[1] == "green" {green = number[0].parse::<i32>().unwrap();}
                if number[1] == "blue" {blue = number[0].parse::<i32>().unwrap();}
            }
            if red > 12 || green > 13 || blue > 14 {
                possible = false;
                break;
            }
        }
        if possible {
            let temp =  input[i].split(": ").collect::<Vec<&str>>()[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            output += temp;
        }
    }

    //Print the output
    println!("Day 2 Output is: {}", output);

}