 
# Fibonacci Numbers CLI tool in Rust
Done as part of practicing the [3.5 Control Flow section of the official Rust documentation](https://doc.rust-lang.org/book/ch03-05-control-flow.html) which has users to try to build certain programs to practice what they learned.

This program can be used via CLI. It prompts to enter a how long would you want the sequence to run and then prints the Fibonacci numbers up to that many times.

## Run Locally  

Clone the project  

~~~bash  
  git clone https://link-to-project
~~~

Go to the project directory  

~~~bash  
  cd rust-cli-fibonacci-numbers
~~~

Build & Run 

~~~bash  
cargo run
~~~

## Demo  

~~~bash  
$ cargo run
   Compiling fibonacci-numbers v0.1.0 (/Users/ahutanu/production/rust/fibonacci-numbers)
    Finished dev [unoptimized + debuginfo] target(s) in 1.32s
     Running `target/debug/fibonacci-numbers`
Please enter up until which Fibonacci number you'd like the sequence to run.
10
Next numbers in the sequence are: 0 & 1.
Fibonacci number 1 is: 1.
Next numbers in the sequence are: 1 & 1.
Fibonacci number 2 is: 1.
Next numbers in the sequence are: 1 & 2.
Fibonacci number 3 is: 2.
Next numbers in the sequence are: 2 & 3.
Fibonacci number 4 is: 3.
Next numbers in the sequence are: 3 & 5.
Fibonacci number 5 is: 5.
Next numbers in the sequence are: 5 & 8.
Fibonacci number 6 is: 8.
Next numbers in the sequence are: 8 & 13.
Fibonacci number 7 is: 13.
Next numbers in the sequence are: 13 & 21.
Fibonacci number 8 is: 21.
Next numbers in the sequence are: 21 & 34.
Fibonacci number 9 is: 34.
Next numbers in the sequence are: 34 & 55.
Fibonacci number 10 is: 55.
~~~

## Playground  

This project is meant as a playground to practice the Rust programming language while following the official documentation.
The author has no previous experience with Rust and this has been developed with knowledge acquired in Rust by following up until the [3.5 Control Flow section of the official Rust documentation](https://doc.rust-lang.org/book/ch03-05-control-flow.html). 
If further concepts that may improve this program at later chapters are learned, this program will not be refactored to use the new concepts, but rather new programs will be built.
Please consider this when reviewing the code.