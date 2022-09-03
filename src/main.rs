use std::io;

fn main() {
    println!("Fibonacci Sequence Generator");
    println!("============================");
    println!("How long of a sequence to generate? ");
    let length = get_input();

    let fibonacci_number = generate_fibonacci_number((length));
    println!("The Fibonacci Number at Position {} is: {}", length, fibonacci_number);
    println!("");
    
    println!("The Fibonacci Full Sequence of length {length}");
    println!("-----------------------------------------------");
    let generated_fibonacci = generate_fibonacci_sequence(length);
    for number in generated_fibonacci {
        println!("{number}");
    }
}

fn get_input() -> u32 {
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("Must enter a lenght");
    let length: u32 = length.trim().parse().expect("Length must be numeric");
    length
}

fn generate_fibonacci_sequence(sequence_length: u32) -> Vec<u64>{
    let mut generated_fibonacci = Vec::new();
    for number in 0..(sequence_length) {
        if number == 0 { 
            generated_fibonacci.push(0 as u64); 
        } else if number == 1 {
            generated_fibonacci.push(1 as u64);
        } else {
            let first_index = number - 2;
            let second_index = number - 1;
            generated_fibonacci.push((generated_fibonacci[first_index as usize] + generated_fibonacci[second_index as usize]) as u64);
        }
    }

    generated_fibonacci
}

fn generate_fibonacci_number(position: u32) -> u64 {
    let mut generated_fibonacci = Vec::new();
    for number in 0..(position) {
        if number == 0 { 
            generated_fibonacci.push(0 as u64); 
        } else if number == 1 {
            generated_fibonacci.push(1 as u64);
        } else {
            let first_index = number - 2;
            let second_index = number - 1;
            generated_fibonacci.push((generated_fibonacci[first_index as usize] + generated_fibonacci[second_index as usize]) as u64);
        }
    }

    generated_fibonacci[(position - 1) as usize]
}