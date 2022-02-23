fn main() {
    fizzbuzz(100);
}

fn fizzbuzz(n: u32) {
    if n > 1 { fizzbuzz(n - 1) }

    if ((n % 3) == 0) && ((n % 5) == 0) {
        println!("FizzBuzz")
    } else if (n % 3) == 0 {
        println!("Fizz")
    } else if (n % 5) == 0 {
        println!("Buzz")
    } else {
        println!("{}", n)
    }
}
