use std::fs;


fn main() {
    let input_string = fs::read_to_string("day/1/input")
        .expect("Something went wrong reading the file");

    let inputs: Vec<i32> = input_string.strip_suffix("\n").unwrap().split("\n").map(|v| v.parse::<i32>().unwrap()).collect();
    let mut increases = 0;

    for (index, input) in inputs.iter().enumerate() {
        if index != 0 {
            if input > &inputs[index-1] {
                increases = increases + 1;
            }
        }
    }
    println!("{}", increases);
}
