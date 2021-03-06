fn main() {
    for x in 1..100 {
        fizz_buzz(x);
    }
}

fn fizz_buzz(x: i32) {
    if x % 3 == 0 && x % 5 == 0 {
        println!("FizzBuzz");
    } else if x % 3 == 0 {
        println!("Fizz");
    } else if x % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", x);
    }
}
