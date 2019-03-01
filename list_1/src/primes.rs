pub fn run_examples() {
    dbg!(get_n_primes(20));
    dbg!(get_n_primes(40));
    dbg!(get_n_primes(100));
}

fn get_n_primes(n: u64) -> String {
    format!("{:?}", return_n_primes(n))
}

fn return_n_primes(n: u64) -> Vec<u64> {
    let mut result = vec![];

    for i in 0..n {
        if check_prime(i) {
            result.push(i);
        }
    }

    result
}

fn check_prime(num: u64) -> bool {
    let is_odd = num > 2 && num % 2 != 0;

    match num == 2 {
        true            => true,
        false if is_odd => is_prime_unchecked(num),
        false           => false,
    }
}

fn is_prime_unchecked(num: u64) -> bool {
    let mut div = 3;
    let stop = (num as f64).sqrt() as u64 + 1;

    while num % div != 0 && div < stop { div += 2; }

    div >= stop
}