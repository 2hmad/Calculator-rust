use std::{
    io::{stdin, stdout, Write},
    result,
};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn main() {
    println!("Welcome to calculator");
    println!("-------------");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    print!("The first number: ");
    read(&mut num1);

    print!("The second number: ");
    read(&mut num2);

    print!("Operation [+-*/]: ");
    read(&mut operator);

    let num1: f32 = num1.trim().parse().unwrap();
    let num2: f32 = num2.trim().parse().unwrap();
    let operator: char = operator.trim().chars().next().unwrap();

    let operators = String::from("+-*/");

    if !operators.contains(operator) {
        println!("unknown operator");
        return;
    }

    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!("Error in operator"),
    };

    println!("the result of {} {} {} = {}", num1, operator, num2, result);
}
