fn main() {
    dbg!(floor(6.21231));
    dbg!(floor(635.21231));
    dbg!(floor(-25673.12341231213));
    dbg!(floor(1.99999999999));
    dbg!(floor(0.99999999999));
    dbg!(floor(-0.000000001));
    dbg!(roof(3455.6564721231));
    dbg!(roof(-25673.8734871));
    dbg!(roof(0.000000000001));
    dbg!(roof(-0.999999999));

}

fn floor(num: f64) -> f64 {
    let sign = num / num.abs();
    let mut result = f64::from(0);

    loop {
        match sign > 0. {
            true  if result > num => return result - 1.,
            false if result < num => return result,
            _                     => result += sign,
        }
    }
}

fn roof(num: f64) -> f64 {
    floor(num) + 1.
}

fn exponent(n: i32) -> f64 {
    f64::from(2).powi(n)
}
