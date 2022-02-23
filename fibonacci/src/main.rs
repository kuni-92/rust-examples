
struct Fibonacci {
    current: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { current: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.current + self.next;
        self.current = self.next;
        self.next = next_value;
        Some(self.current)
    }
}

fn main() {
    let fibonacci = Fibonacci::new();
    for num in fibonacci.take(10) {
        println!("fibonacci value: {}", num);
    }
}
