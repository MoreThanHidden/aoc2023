use std::fs::File;
use std::io::Read;

pub fn day6() {
    //Take input from a text File
    let mut file = File::open("day6.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split("\n").collect();

    //time from input
    let time = input[0].split(":").collect::<Vec<&str>>()[1].trim().split_whitespace().collect::<Vec<&str>>();
    let distance = input[1].split(":").collect::<Vec<&str>>()[1].trim().split_whitespace().collect::<Vec<&str>>();

    //Print the output
    println!("Day 6 / Part 1 Output is: {}", check_races(time, distance));


    //time from input
    let time = input[0].split(":").collect::<Vec<&str>>()[1].trim().replace(" ","");
    let time2  = [time.as_str()].into_iter().collect::<Vec<&str>>();

    let distance = input[1].split(":").collect::<Vec<&str>>()[1].trim().replace(" ","");
    let distance2 = [distance.as_str()].into_iter().collect::<Vec<&str>>();

    //Print the output
    println!("Day 6 / Part 2 Output is: {}", check_races(time2, distance2));

}

fn check_races (time : Vec<&str>, distance : Vec<&str>) -> i64 {
    let mut out = 1;
    for i in 0..time.len() {
        let time = time[i].parse::<i64>().unwrap();
        let distance = distance[i].parse::<i64>().unwrap();

        let mut ways_to_win = 0;
        let mut button_held = 0;

        while button_held < time {
            let distance_traveled = (time - button_held) * button_held;
            if distance_traveled > distance {
                ways_to_win += 1;
            }
            button_held += 1;
        }
        out *= ways_to_win;
    }
    return out;
}