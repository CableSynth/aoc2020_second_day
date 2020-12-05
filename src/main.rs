use std::fs;

fn main() {

    let filename = "/home/caleb/codestuff/advent2020/second_day/src/input.txt";
    let data =  fs::read_to_string(filename).expect("error");
    let data_split_on_newline: Vec<&str> = data.split("\n").collect();
    let mut total = 0;
    for line in data_split_on_newline {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        // println!("{:?}", parts);
        let numbers: Vec<&str> = parts[0].split("-").collect();
        let first_num = numbers[0].parse::<i64>().expect("badvalue");
        let second_num = numbers[1].parse::<i64>().expect("badvalue");
        let letter_needed = parts[1].chars().nth(0).unwrap();
        let mut count = 0;
        for chr in parts[2].chars() {
            // println!("{}", chr);
            if chr == letter_needed {
                count = count + 1;
            }
        }

        if first_num <= count && second_num >= count {
            total = total + 1;
        }
    }
    println!("{}", total);

    let data_split_on_newline: Vec<&str> = data.split("\n").collect();
    let mut total = 0;

    for line in data_split_on_newline {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        // println!("{:?}", parts);
        let numbers: Vec<&str> = parts[0].split("-").collect();
        let first_num = numbers[0].parse::<usize>().expect("badvalue");
        let second_num = numbers[1].parse::<usize>().expect("badvalue");
        let letter_needed = parts[1].chars().nth(0).unwrap();
        let carac = parts[2].as_bytes();
        let mut exists = false;
        if carac[first_num - 1] == letter_needed as u8 {
            exists = !exists;
        } 
        if carac[second_num - 1] == letter_needed as u8 {
            exists = !exists;
        }

        if exists {
            total = total + 1;
        }
    } 
    println!("{}", total);
}
