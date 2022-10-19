use std::io;

fn main() {
    let mut f_n_minus_1: i128 = 0; //first
    let mut f_n_minus_2: i128 = 1; //second

    let sequence_run_times: i32 = loop {
        println!("Please enter up until which Fibonacci number you'd like the sequence to run.");

        let mut sequence_run_times: String = String::new();

        io::stdin()
            .read_line(&mut sequence_run_times)
            .expect("Failed to read line");

        match sequence_run_times.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    let mut iterations: i32 = 0;
    while iterations < sequence_run_times {
        println!("Next numbers in the sequence are: {f_n_minus_1} & {f_n_minus_2}.");

        let current_number: i128 = f_n_minus_1 + f_n_minus_2;

        println!("Fibonacci number {} is: {f_n_minus_2}.", iterations+1);

        f_n_minus_1 = f_n_minus_2;
        f_n_minus_2 = current_number;
        iterations += 1;
    }
}
