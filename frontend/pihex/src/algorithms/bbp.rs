use wasm_bindgen::prelude::*;
use rayon::prelude::*;
use super::utils::mod_pow;

#[wasm_bindgen]
pub fn bbp(n: usize) -> String {

    // The BBP approximation of pi can be broken down into four sums, each of which has 
    // the following form: num_const * \Sigma_0^inf (1/(16^k(8k + denom_const)))
    let sum: f64 = [
        (4.0, 1), 
        (-2.0, 4), 
        (-1.0, 5), 
        (-1.0, 6)
    ].par_iter()
    .map(|(num_const, denom_const)| {
        num_const * compute_sum(n, *denom_const) 
    }).sum();

    format!("{:x}", ((sum - sum.floor()) * 16.0).floor() as usize)
}

fn compute_sum(digit: usize, denom_const: usize) -> f64 {

    // We can split the sum into two parts: a finite sum and an infinite sum.
    
    // The finite sum can be kept small by using modular exponentiation.
    let finite_sum = (0..=digit).fold(0.0, |sum, k| {
        let denominator = 8 * k + denom_const;
        let summand = (mod_pow(16, digit - k, denominator) as f64) / (denominator as f64);
        (sum + summand).fract()
    });

    let mut sixteen_pow = 1.0;

    // The infinite sum can be computed by summing terms until the summand is less than epsilon.
    let infinite_sum = (digit+1..).map(|k| {
        sixteen_pow *= 1.0/16.0;
        sixteen_pow / ((8 * k + denom_const) as f64)
    }).take_while(|&summand| 
        summand > f64::EPSILON
    ).sum::<f64>();

    return (finite_sum + infinite_sum).fract();
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils::valid_hex_digit_compute_fn;

    #[test]
    fn test_bbp() {
        valid_hex_digit_compute_fn(bbp);
    }
}

