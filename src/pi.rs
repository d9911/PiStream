use indicatif::{ProgressBar, ProgressStyle};
use rug::{Float, ops::Pow};

pub fn compute_pi(digits: usize) -> String {
    // Calculate required precision in bits (approximately digits * log2(10))
    let prec = ((digits as f64) * 3.3219280948873626).ceil() as u32 + 10; // extra bits for safety

    // Initialize sum
    let mut sum = Float::with_val(prec, 0);

    // Constants for Chudnovsky algorithm
    let mut a = Float::with_val(prec, 13591409);
    let mut factorial = Float::with_val(prec, 1);
    let mut power_of_640320_cubed = Float::with_val(prec, 1);
    let base_640320_cubed = Float::with_val(prec, 640320).pow(3);

    // Estimate number of iterations (rough approximation)
    let max_k = (digits / 14) + 10;
    let pb = ProgressBar::new(max_k as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut k = 0;
    loop {
        // Compute term: (-1)^k * a / (b * d)
        let sign = if k % 2 == 0 { 1 } else { -1 };
        let denominator = Float::with_val(prec, &factorial * &power_of_640320_cubed);
        let numerator = Float::with_val(prec, sign) * &a;
        let term = numerator / denominator;
        sum += &term;

        // Check if term is small enough
        let threshold = Float::with_val(prec, 10).pow(-(digits as i32));
        if term.abs() < threshold {
            break;
        }

        // Update for next k
        k += 1;
        a += 545140134;

        // Update factorial: (6k)! / (3k)! / k!^3
        factorial *= (6 * k - 5) * (6 * k - 4) * (6 * k - 3) * (6 * k - 2) * (6 * k - 1) * (6 * k);
        factorial /= (3 * k - 2) * (3 * k - 1) * (3 * k);
        factorial /= k * k * k;

        // Update power of 640320^3
        power_of_640320_cubed *= &base_640320_cubed;

        pb.set_position(k as u64);
        if k >= max_k {
            pb.println("Warning: Reached maximum iterations, precision may be insufficient.");
            break;
        }
    }

    pb.finish_with_message("Computation complete");

    // Compute pi using Chudnovsky formula: pi = 426880 * sqrt(10005) / sum
    let sqrt_10005 = Float::with_val(prec, 10005).sqrt();
    let pi = Float::with_val(prec, 426880) * sqrt_10005 / sum;

    // Convert to string with specified digits
    pi.to_string_radix(10, Some(digits + 1))
}
