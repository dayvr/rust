fn fibonacci_1(n: i32) -> i32 {
    if n <= 0 {
        0
    }
    else if n == 1 {
        1
    }
    else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn fibonacci_2(m: i32) -> i32 {
    match m {
        0 =>  0,
        1 =>  1,
        _ => fibonacci(m - 1) + fibonacci(m - 2),
    }
}


fn main(){
    println!("fibonacci(4)={}",fibonacci_1(4));
    println!("fibonacci(9)={}",fibonacci_2(9));
}
