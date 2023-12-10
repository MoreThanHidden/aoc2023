use std::fs::File;
use std::io::Read;
use std::ops::Range;
use rayon::prelude::*;

pub fn day5() {
    //Take input from a text File
    let mut file = File::open("day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split(":").collect();


    let seeds = input[1].split("\n\n").collect::<Vec<&str>>()[0].trim().split_whitespace().collect::<Vec<&str>>();

    let mut seed_to_soil = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temperature = Vec::new();
    let mut temperature_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();

    //Loop through the maps
    for i in 1..input.len(){
        let temp = input[i].split("\n\n").collect::<Vec<&str>>();
        if temp.len() > 1 && temp[1] == "seed-to-soil map" {
            process_map("seed-to-soil map", &input, i, &mut seed_to_soil);
        }else if temp.len() > 1 && temp[1] == "soil-to-fertilizer map"{
            process_map("soil-to-fertilizer map", &input, i, &mut soil_to_fertilizer);
        }else if temp.len() > 1 && temp[1] == "fertilizer-to-water map"{
            process_map("fertilizer-to-water map", &input, i, &mut fertilizer_to_water);
        }else if temp.len() > 1 && temp[1] == "water-to-light map"{
            process_map("water-to-light map", &input, i, &mut water_to_light);
        }else if temp.len() > 1 && temp[1] == "light-to-temperature map"{
            process_map("light-to-temperature map", &input, i, &mut light_to_temperature);
        }else if temp.len() > 1 && temp[1] == "temperature-to-humidity map"{
            process_map("temperature-to-humidity map", &input, i, &mut temperature_to_humidity);
        }else if temp.len() > 1 && temp[1] == "humidity-to-location map"{
            process_map("humidity-to-location map", &input, i, &mut humidity_to_location);
        }
    }

    let mut min_location = i64::MAX;

    let maps = [&seed_to_soil, &soil_to_fertilizer, &fertilizer_to_water, &water_to_light, &light_to_temperature, &temperature_to_humidity, &humidity_to_location];

    // Loop through the seeds
    for seed in &seeds {
        let seed = seed.parse::<i64>().unwrap();
        let location = get_location(seed, &maps);
        if location < min_location {
            min_location = location;
        }
    }

    println!("Day 5 / Part 1 Output is: {}", min_location);

    //Part 2 Seed Ranges
    let mut min_location = i64::MAX;

    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i].parse::<i64>().unwrap();
        let end = start + seeds[i+1].parse::<i64>().unwrap();
        let min_location_in_range = min_location_in_range(start, end, &maps);
        if min_location_in_range < min_location {
            min_location = min_location_in_range;
        }
    }

    println!("Day 5 / Part 2 Output is: {}", min_location);

}

fn process_map(map_name: &str, input: &[&str], i: usize, result_vec: &mut Vec<(Range<i64>, i64)>) {
    let data = input[i+1].split("\n\n").collect::<Vec<&str>>()[0];
    let map = data.trim().split("\n").collect::<Vec<&str>>();
    for j in 0..map.len(){
        let temp = map[j].split(" ").collect::<Vec<&str>>();
        let source = temp[1].parse::<i64>().unwrap();
        let diff = temp[0].parse::<i64>().unwrap() - source;
        let amount = temp[2].parse::<i64>().unwrap();
        result_vec.push((source..source+amount, diff));
    }
    result_vec.sort_by_key(|k| k.0.start);
}

fn get_location(seed: i64, maps: &[&Vec<(Range<i64>, i64)>]) -> i64 {
    let mut location = seed;

    for map in maps {
        for (range, diff) in map.iter() {
            if range.contains(&location) {
                location += diff;
                break;
            }
        }
    }

    location
}

fn min_location_in_range(start: i64, end: i64, maps: &[&Vec<(Range<i64>, i64)>]) -> i64 {
    let min_location = (start..end).into_par_iter()
        .map(|seed| get_location(seed, maps))
        .min()
        .unwrap_or(i64::MAX);

    min_location
}