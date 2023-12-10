use std::fs::File;
use std::io::Read;

pub fn day4() {
    //Take input from a text File
    let mut file = File::open("day4.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split("\n").collect();

    //Create a variable to store the output
    let mut out = 0;

    let mut cards = Vec::new();

    for line in input {
        let temp = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = temp[0].split_whitespace().collect::<Vec<&str>>();
        let my_numbers = temp[1].split_whitespace().collect::<Vec<&str>>();

        //count the number of winning numbers
        let mut winning_numbers_count = 0;
        for number in winning_numbers {
            if my_numbers.contains(&number) {
                winning_numbers_count += 1;
            }
        }
        //out += The first match makes the card worth one point and each match after the first doubles the point value of that card.
        if winning_numbers_count > 0 {
            let mut out2 = 1;
            for _i in 0..winning_numbers_count-1 {
                out2 *= 2;
            }
            out += out2;
        }

        //part 2 add cards to the vec
        let card_number = line.split(":").collect::<Vec<&str>>()[0].trim().split_whitespace().collect::<Vec<&str>>()[1];
        cards.push((card_number.parse::<i32>().unwrap(), winning_numbers_count));

    }

    let mut out_part2 = 0;
    for card in &cards {
       out_part2 += card_loop(card.0, card.1, &cards);
    }

    //Print the output
    println!("Day 4 / Part 1 Output is: {}", out);
    println!("Day 4 / Part 2 Output is: {}", out_part2);

}

fn card_loop(card: i32, wins: i32, cards: &Vec<(i32, i32)>) -> i32 {
    let mut out = 1;
    for i in card..card+wins {
        if(i < cards.len() as i32){
            out += card_loop(cards[i as usize ].0, cards[i as usize ].1, cards);
        }
    }
    return out;
}
