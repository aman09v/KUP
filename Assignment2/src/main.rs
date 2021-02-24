use std::io;

fn main() {
    println!("Hello, world!");
    let mut n = String::new();
    print!("Enter the length of series ");
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let num = n.trim().parse().expect("invalid input");
    fibonacci(num);
}

fn fibonacci(num: i32){
    let mut n1: i32 = 0;
    let mut n2: i32 = 1;
    let mut next: i32 = 0;
    if num <= 0 {
        print!("Please enter a positive integer");
    }
    else if num == 1 {
        print!("{}",n1);
    }
    else{
        for x in 0..num {
            print!("{} ", n1);
            next = n1+n2;
            n1=n2;
            n2=next;
        }
    }
}
