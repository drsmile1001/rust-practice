fn main() {
    for s in 0..10 {
        println!("{}", get_fibonacci_number(s))
    }
}

fn get_fibonacci_number(s: i32) -> i32 {
    if s < 0 {
        panic!("s must > 0")
    }

    if s == 0 {
        return 0;
    } else if s == 1 {
        return 1;
    }
    get_fibonacci_number(s - 1) + get_fibonacci_number(s - 2)
}
