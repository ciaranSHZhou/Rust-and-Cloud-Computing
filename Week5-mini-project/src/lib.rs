//Write a function takes a number and returns all the prime numbers up to that number
pub fn prime_numbers(num: i32) -> Vec<i32> {
    let mut primes = Vec::new();
    for i in 2..num {
        if is_prime(i) {
            primes.push(i);
        }
    }
    primes
}
//Write a function that takes a number and returns true if it is prime and false if it is not
pub fn is_prime(num: i32) -> bool {
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}
