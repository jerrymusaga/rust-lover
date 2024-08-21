/*
Write a program to find the sum of natural numbers below a given number N, where N is provided by the user.
The sum should only include numbers that are multiples of either 3 or 5.
For example, if the user enters N = 20, the multiples of 3 are 3, 6, 9, 12, 15, 18, and the multiples of 5 are 5, 10, and 15.

Please note that the value of 15 will be considered only once since it is a multiple of both 3 and 5.
The sum will be calculated as follows: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18.

Write a program that takes the user input N, performs the necessary calculations, and outputs the sum.
*/

pub fn result() {
    let mut input = String::new();
    println!("Input the value");
    std::io::stdin().read_line(&mut input).expect("cannot read line");
    let n: u32 = match input.trim().parse(){
        Ok(num) => {
            println!("You inputted {num}");
            num
        },
        Err(_) => {
            println!("Please input a number");
            0
        }
    };
    let mut sum_of_3 = 0;
    let mut sum_of_5 = 0;
    for x in 1..n{
        if x % 3 == 0{
            sum_of_3 += x;
            println!("sum in the multiples of 3:{sum_of_3}");
        }
        else if x % 5 == 0 {
            sum_of_5 += x;
            println!("sum in the multiples of 5:{sum_of_5}");
        }
    }
    println!("The sum is {}", sum_of_3 + sum_of_5);
}