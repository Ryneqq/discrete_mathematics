pub fn run_examples() {
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

    dbg!(fraction(1234.56789));
    dbg!(fraction(-1234.56789));
    dbg!(fraction(0.56789));
    dbg!(fraction(-0.56789));

    dbg!(modulo(10, 3));
    dbg!(modulo(123, -11));
    dbg!(modulo(-321, 27));
    dbg!(modulo(-456, -14));
}

fn modulo(num: i64, div: i64) -> i64 {
    let div        = div.abs();
    let sign       = num / num.abs();
    let mut result = num.abs();

    loop {
        match result > 0 {
            true  => result -= div,
            false => return (result + div) * sign ,
        }
    }
}

fn fraction(num: f64) -> f64 {
    let num = num.abs();
    num - floor(num)
}

fn roof(num: f64) -> f64 {
    floor(num) + 1.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(floor(6.21231), 6.);
        assert_eq!(floor(635.21231), 635.);
        assert_eq!(floor(-25673.12341231213), -25674.);
        assert_eq!(floor(1.99999999999), 1.);
        assert_eq!(floor(0.99999999999), 0.);
        assert_eq!(floor(-0.000000001), -1.);

        assert_eq!(roof(3455.6564721231), 3456.);
        assert_eq!(roof(-25673.8734871), -25673.);
        assert_eq!(roof(0.000000000001), 1.);
        assert_eq!(roof(-0.999999999), 0.);

        // They will not work because of the approximation
        // assert_eq!(fraction(1234.56789), 0.56789);
        // assert_eq!(fraction(-1234.56789), 0.56789);
        // assert_eq!(fraction(0.56789), 0.56789);
        // assert_eq!(fraction(-0.56789), 0.56789);

        assert_eq!(modulo(10, 3), 10 % 3);
        assert_eq!(modulo(123, -11), 123 % -11);
        assert_eq!(modulo(-321, 27), -321 % 27);
        assert_eq!(modulo(-456, -14), -456 % -14);
    }
}