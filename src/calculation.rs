//! Checked, cancellable implementations shared by the desktop and CLI adapters.
use crate::{Algorithm, Digits, Precision};
use rug::{ops::Pow, Float, Integer};
use std::sync::atomic::{AtomicBool, Ordering};

pub const DESKTOP_MAX_DIGITS: usize = 1_000_000;

#[derive(Default)]
pub struct Cancellation(AtomicBool);

impl Cancellation {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
    fn check(&self) -> Result<(), String> {
        if self.is_cancelled() {
            Err("Вычисление остановлено".into())
        } else {
            Ok(())
        }
    }
}

pub fn parse_digits(input: &str) -> Result<usize, String> {
    let compact: String = input
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '_')
        .collect();
    let digits: usize = compact
        .parse()
        .map_err(|_| "Введите целое число знаков, например 1000".to_string())?;
    if !(1..=DESKTOP_MAX_DIGITS).contains(&digits) {
        return Err("Выберите от 1 до 1 000 000 знаков после запятой".into());
    }
    Ok(digits)
}

pub fn compute(
    algorithm: Algorithm,
    digits: usize,
    precision: Precision,
    cancel: &Cancellation,
) -> Result<String, String> {
    cancel.check()?;
    if digits == 0 || !algorithm.can_handle_digits(digits) || digits > 1_000_000_000 {
        return Err("Слишком большое количество знаков или нулевая точность".into());
    }
    let bits = precision.bits();
    let pi = match algorithm {
        Algorithm::ChudnovskyBinarySplitting => binary(digits, bits, cancel)?,
        Algorithm::ChudnovskyNaive => series(digits, bits, cancel)?,
        Algorithm::MachinFormula => machin(digits, bits, cancel)?,
        Algorithm::BorweinQuadratic => agm(digits, bits, cancel)?,
        _ => return Err("Этот режим доступен в консольной версии".into()),
    };
    cancel.check()?;
    if !pi.is_finite() || pi <= 3 || pi >= 4 {
        return Err("Не удалось получить корректный результат π".into());
    }
    Ok(pi.to_string_radix(10, Some(digits + 1)))
}

pub fn compute_default(
    algorithm: Algorithm,
    digits: usize,
    cancel: &Cancellation,
) -> Result<String, String> {
    let digits = Digits::new(digits)?;
    compute(
        algorithm,
        digits.value(),
        Precision::from_digits(digits),
        cancel,
    )
}

/// Exact leaf shared by all Chudnovsky binary-splitting adapters.
pub(crate) fn chudnovsky_term(index: usize) -> (Integer, Integer, Integer) {
    if index == 0 {
        return (
            Integer::from(1),
            Integer::from(1),
            Integer::from(13_591_409),
        );
    }
    let k = Integer::from(index);
    let p: Integer =
        (Integer::from(&k * 6) - 5) * (Integer::from(&k * 2) - 1) * (Integer::from(&k * 6) - 1);
    let q = k.clone().pow(3) * Integer::from(10_939_058_860_032_000_u64);
    let mut t: Integer = p.clone() * (Integer::from(545_140_134) * &k + 13_591_409);
    if index % 2 == 1 {
        t = -t;
    }
    (p, q, t)
}

fn binary(digits: usize, bits: u32, cancel: &Cancellation) -> Result<Float, String> {
    fn split(
        a: usize,
        b: usize,
        cancel: &Cancellation,
    ) -> Result<(Integer, Integer, Integer), String> {
        cancel.check()?;
        if b - a == 1 {
            return Ok(chudnovsky_term(a));
        }
        let mid = a + (b - a) / 2;
        let (p1, q1, t1) = split(a, mid, cancel)?;
        let (p2, q2, t2) = split(mid, b, cancel)?;
        let t = Integer::from(&t1 * &q2) + Integer::from(&p1 * &t2);
        Ok((p1 * p2, q1 * q2, t))
    }
    let (_, q, t) = split(0, digits / 14 + 3, cancel)?;
    cancel.check()?;
    Ok(Float::with_val(bits, 426_880)
        * Float::with_val(bits, 10_005).sqrt()
        * Float::with_val(bits, q)
        / Float::with_val(bits, t))
}

fn series(digits: usize, bits: u32, cancel: &Cancellation) -> Result<Float, String> {
    let mut sum = Float::with_val(bits, 0);
    let mut numerator = Integer::from(1);
    let mut denominator = Integer::from(1);
    let mut power = Integer::from(1);
    let base = Integer::from(640_320).pow(3);
    for k in 0..(digits / 14 + 3) {
        cancel.check()?;
        let coefficient = Integer::from(k) * 545_140_134 + 13_591_409;
        let mut term = Float::with_val(bits, &numerator) * Float::with_val(bits, coefficient);
        term /= Float::with_val(bits, Integer::from(&denominator * &power));
        if k % 2 == 0 {
            sum += term;
        } else {
            sum -= term;
        }
        let next = k + 1;
        for i in 0..6 {
            numerator *= 6 * next - 5 + i;
        }
        denominator *= Integer::from(3 * next - 2)
            * Integer::from(3 * next - 1)
            * Integer::from(3 * next)
            * Integer::from(next).pow(3);
        power *= &base;
    }
    Ok(Float::with_val(bits, 426_880) * Float::with_val(bits, 10_005).sqrt() / sum)
}

fn machin(digits: usize, bits: u32, cancel: &Cancellation) -> Result<Float, String> {
    fn arctan(
        divisor: u32,
        digits: usize,
        bits: u32,
        cancel: &Cancellation,
    ) -> Result<Float, String> {
        let x = Float::with_val(bits, 1) / divisor;
        let square = Float::with_val(bits, &x * &x);
        let mut power = x;
        let mut sum = Float::with_val(bits, 0);
        let threshold = Float::with_val(bits, 1) >> (bits.saturating_sub(16) as i32);
        for n in 0..(digits + 50) {
            cancel.check()?;
            let term = Float::with_val(bits, &power) / (2 * n + 1);
            if n % 2 == 0 {
                sum += &term;
            } else {
                sum -= &term;
            }
            if term < threshold {
                break;
            }
            power *= &square;
        }
        Ok(sum)
    }
    Ok(Float::with_val(bits, 16) * arctan(5, digits, bits, cancel)?
        - Float::with_val(bits, 4) * arctan(239, digits, bits, cancel)?)
}

fn agm(digits: usize, bits: u32, cancel: &Cancellation) -> Result<Float, String> {
    let mut a = Float::with_val(bits, 1);
    let mut b = Float::with_val(bits, 2).sqrt().recip();
    let mut t = Float::with_val(bits, 0.25);
    let mut p = Float::with_val(bits, 1);
    for _ in 0..((digits as f64).log2().ceil() as usize + 5) {
        cancel.check()?;
        let next_a = Float::with_val(bits, &a + &b) / 2;
        let next_b = Float::with_val(bits, &a * &b).sqrt();
        let difference = Float::with_val(bits, &a - &next_a);
        t -= Float::with_val(bits, &p) * difference.pow(2);
        a = next_a;
        b = next_b;
        p *= 2;
        if a == b {
            break;
        }
    }
    Ok(Float::with_val(bits, &a + &b).pow(2) / (Float::with_val(bits, 4) * t))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn algorithms_agree_with_independent_mpfr_pi() {
        for digits in [1, 5, 10, 50, 100, 1000] {
            let precision = Precision::from_digits(Digits::new(digits).expect("valid precision"));
            let expected = Float::with_val(precision.bits() + 128, rug::float::Constant::Pi)
                .to_string_radix(10, Some(digits + 1));
            for algorithm in [
                Algorithm::ChudnovskyNaive,
                Algorithm::ChudnovskyBinarySplitting,
                Algorithm::MachinFormula,
                Algorithm::BorweinQuadratic,
            ] {
                assert_eq!(
                    compute(algorithm, digits, precision, &Cancellation::default())
                        .expect("calculation"),
                    expected,
                    "{algorithm:?}: {digits} digits"
                );
            }
        }
    }
    #[test]
    fn cancelled_calculation_returns_an_error() {
        let cancel = Cancellation::default();
        cancel.cancel();
        assert_eq!(
            compute_default(Algorithm::ChudnovskyBinarySplitting, 1000, &cancel)
                .expect_err("cancelled"),
            "Вычисление остановлено"
        );
    }
    #[test]
    fn desktop_precision_is_validated_before_work_starts() {
        for invalid in ["", "0", "-5", "1.5", "1000001", "abc"] {
            assert!(parse_digits(invalid).is_err());
        }
        assert_eq!(parse_digits("1 000"), Ok(1000));
        assert_eq!(parse_digits("1_000_000"), Ok(1_000_000));
    }
}
