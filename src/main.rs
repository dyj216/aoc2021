use std::collections::{HashMap, HashSet};
use std::fs;
use std::str;
use std::env;
use std::process;

fn read_input_file(path: &str) -> Vec<String> {
    let input_string = fs::read_to_string(path).expect("Something went wrong reading the file");
    let inputs: Vec<String> = input_string.strip_suffix("\n").unwrap().split("\n").map(|s| s.to_string()).collect();
    return inputs;
}

fn convert_string_inputs_to_int(inputs: Vec<String>) -> Vec<i32> {
    return inputs.iter().map(|v| v.parse::<i32>().unwrap()).collect();
}

fn solve_day_1(task: i8, raw_inputs: Vec<String>) -> i32 {
    let inputs: Vec<i32> = convert_string_inputs_to_int(raw_inputs);

    match task {
        1 => {
            let mut increases = 0;
            for (index, input) in inputs.iter().enumerate() {
                if index != 0 {
                    if input > &inputs[index - 1] {
                        increases = increases + 1;
                    }
                }
            }
            increases
        }
        2 => {
            let mut increases = -1;
            let mut previous_sum = 0;

            for (index, input) in inputs.iter().enumerate() {
                if index > 1 {
                    if (*input + inputs[index - 1] + inputs[index - 2]) > previous_sum {
                        increases = increases + 1;
                    }
                    previous_sum = *input + inputs[index - 1] + inputs[index - 2];
                }
            }
            increases
        }
        _ => panic!("incorrect task")
    }
}

fn solve_day_2(task: i8, raw_inputs: Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    for movement in raw_inputs {
        let mut iter = movement.split_whitespace();
        let direction = iter.next();
        let amount = iter.next().unwrap().parse::<i32>().unwrap();
        if task == 1 {
            match direction {
                Some("forward") => horizontal += amount,
                Some("up") => vertical -= amount,
                Some("down") => vertical += amount,
                _ => panic!("incorrect input")
            }
        } else {
            match direction {
                Some("forward") => {
                    horizontal += amount;
                    vertical = vertical + aim * amount;
                }
                Some("up") => aim -= amount,
                Some("down") => aim += amount,
                _ => panic!("incorrect input")
            }
        }
    }
    return horizontal * vertical;
}

fn solve_day_3(task: i8, raw_inputs: Vec<String>) -> i32 {
    let inputs_size = raw_inputs.len() as i32;
    let mut bitcount_at_positions = count_bits_in_binary_inputs(&raw_inputs);
    match task {
        1 => {
            let gamma = bitcount_at_positions.iter().map(|x| if x > &(inputs_size / 2) { 1 } else { 0 }).fold(0, |acc, bit| (acc << 1) ^ bit);
            let epsilon = bitcount_at_positions.iter().map(|x| if x < &(inputs_size / 2) { 1 } else { 0 }).fold(0, |acc, bit| (acc << 1) ^ bit);
            return gamma * epsilon;
        }
        2 => {
            let mut reduced_for_o2 = raw_inputs.clone();
            let mut reduced_for_co2 = raw_inputs.clone();
            let original_bitcount_at_positions = bitcount_at_positions.clone();

            let mut i = 0;
            while reduced_for_o2.len() != 1 {
                if bitcount_at_positions[i] >= if reduced_for_o2.len() % 2 == 0 { reduced_for_o2.len() / 2 } else { (reduced_for_o2.len() + 1) / 2 } as i32 {
                    reduced_for_o2.retain(|x| filter_bits(x, i, '1'))
                } else {
                    reduced_for_o2.retain(|x| filter_bits(x, i, '0'))
                }
                bitcount_at_positions = count_bits_in_binary_inputs(&reduced_for_o2);
                i += 1;
            }
            i = 0;
            bitcount_at_positions = original_bitcount_at_positions;
            while reduced_for_co2.len() != 1 {
                if bitcount_at_positions[i] >= if reduced_for_co2.len() % 2 == 0 { reduced_for_co2.len() / 2 } else { (reduced_for_co2.len() + 1) / 2 } as i32 {
                    reduced_for_co2.retain(|x| filter_bits(x, i, '0'))
                } else {
                    reduced_for_co2.retain(|x| filter_bits(x, i, '1'))
                }
                bitcount_at_positions = count_bits_in_binary_inputs(&reduced_for_co2);
                i += 1;
            }
            return isize::from_str_radix(reduced_for_o2[0].as_str(), 2).unwrap() as i32
                * isize::from_str_radix(reduced_for_co2[0].as_str(), 2).unwrap() as i32;
        }
        _ => panic!("incorrect task")
    }
}

fn filter_bits(bits: &String, position: usize, bit_to_keep: char) -> bool {
    if bits.chars().nth(position).unwrap() == bit_to_keep {
        return true;
    }
    return false;
}

fn count_bits_in_binary_inputs(inputs: &Vec<String>) -> Vec<i32> {
    let mut result = vec![0; 12];
    for input in inputs {
        for (index, bit) in input.chars().enumerate() {
            match bit {
                '1' => result[index] += 1,
                _ => ()
            }
        }
    }
    return result;
}

// fn solve_test(task: i8, raw_inputs: Vec<String>) -> i32 {
//     let mut a = vec![1, 2, 3];
//     for i in &mut a {
//         // iterate mutably
//         let i: &mut i32 = i; // elements are mutable pointers
//         *i *= 2;
//         println!("{}", i);
//     }
//     return 5
// }

fn solve_day_4(task: i8, raw_inputs: Vec<String>) -> i32 {
    if task != 1 && task != 2 {panic!("incorrect task")}
    let mut winner_number: i32 = 0;
    let mut winner_table: i32 = 0;
    let (bingo_numbers, mut bingo_tables) = process_bingo_data(raw_inputs);
    let mut won_tables: HashSet<i32> = HashSet::new();
    'bingo: for bingo_number in bingo_numbers {
        for (_, bingo_table) in &mut bingo_tables {
            for bingo_line in &mut *bingo_table {
                for n in &mut *bingo_line {
                    if *n == bingo_number {
                        *n = 0;
                    }
                }
            }
        }
        for (table_number, bingo_table) in &bingo_tables {
            for bingo_line in bingo_table {
                if bingo_line.iter().all(|&x| x == 0) {
                    winner_number = bingo_number;
                    winner_table = *table_number;
                    if task == 1 {break 'bingo;}
                    if task == 2 {
                        won_tables.insert(winner_table);
                        if bingo_tables.keys().all(|k| won_tables.contains(&k)) {
                            break 'bingo;
                        }
                    }
                }
            }
            for index  in 0..5 {
                let sum_of_col = &bingo_table.iter().fold(0, |acc, v| acc + v[index]);
                if *sum_of_col == 0 {
                    winner_number = bingo_number;
                    winner_table = *table_number;
                    if task == 1 {break 'bingo;}
                    if task == 2 {
                        won_tables.insert(winner_table);
                        if bingo_tables.keys().all(|k| won_tables.contains(&k)) {
                            break 'bingo;
                        }
                    }
                }
            }
        }
    }
    let mut leftover_sum = 0;
    for winner_lines in &bingo_tables[&winner_table] {
        leftover_sum += winner_lines.iter().sum::<i32>();
    }
    return leftover_sum * winner_number;
}

fn process_bingo_data(raw_inputs: Vec<String>) -> (Vec<i32>, HashMap<i32, Vec<Vec<i32>>>) {
    let bingo_numbers: Vec<i32> = raw_inputs[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut bingo_tables: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();

    let mut table_number = 0;
    for (index, line) in raw_inputs[2..].iter().enumerate() {
        if index % 6 == 0 {
            table_number = (index / 6) as i32;
            bingo_tables.insert(table_number, vec![]);
        }
        if index % 6 < 5 {
            bingo_tables.get_mut(&table_number).unwrap().push(line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect())
        }
    }
    return (bingo_numbers, bingo_tables)
}

fn main() {
    let selector = DayTaskSelector::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let raw_inputs = read_input_file(&format!("day/{}/input", selector.day));
    match selector.day {
        // 0 => println!("{}", solve_test(selector.task, raw_inputs)),
        1 => println!("{}", solve_day_1(selector.task, raw_inputs)),
        2 => println!("{}", solve_day_2(selector.task, raw_inputs)),
        3 => println!("{}", solve_day_3(selector.task, raw_inputs)),
        4 => println!("{}", solve_day_4(selector.task, raw_inputs)),
        _ => println!("Not implemented (yet)")
    }
}

struct DayTaskSelector {
    day: i8,
    task: i8,
}

impl DayTaskSelector {
    pub fn new(mut args: env::Args) -> Result<DayTaskSelector, &'static str> {
        args.next();

        let day = match args.next() {
            Some(arg) => arg.parse::<i8>().unwrap(),
            None => return Err("Didn't get a day number")
        };
        let task = match args.next() {
            Some(t) if (t == "1" || t == "2") => t.parse::<i8>().unwrap(),
            _ => return Err("Task number can be 1 or 2")
        };

        Ok(DayTaskSelector { day, task })
    }
}
