fn main() {
    println!("Fibonacci");

    let size = 100;

    let numbers = generate(size);
    println!("{} Fibonacci numbers: {:?}", size, numbers);
}

fn generate (size: usize) -> Vec<u64> {
    let mut numbers: Vec<u64> = Vec::new();

    for i in 0..size {
        numbers.push(fibonacci(i));
    }
    numbers
}

fn fibonacci(n: usize) -> u64 {
    if n == 0 {
        return 0_u64;
    } else if n == 1 {
        return 1_u64;
    }
 
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    
    for _ in 2..=n {
        let c = a.saturating_add(b);
        a = b;
        b = c;
    }
    b
}
