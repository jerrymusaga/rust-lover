// Problem 1:
/*
Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers.
N will be a user-defined input that your program will take.

For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.
Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.
Finally, calculate the difference as 225 - 55 = 170.
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
    let mut square_of_sum = 0;
    let mut sum_of_square = 0;
    for x in 1..=n{
        square_of_sum += x;
        sum_of_square += x.pow(2);
    }
    square_of_sum = square_of_sum.pow(2);
    println!("Square of sum is: {square_of_sum}");
    println!("Sum of square is: {sum_of_square}");

    let difference = square_of_sum - sum_of_square;
    println!("Difference is {difference}");
    
    
}