use wasm_bindgen::prelude::*;
use rayon::prelude::*;
use super::utils::mod_pow;

#[wasm_bindgen]
pub fn bellard(n: usize) -> String {

    // We use the same approach as the BBP algorithm, see comments in bbp.rs for more details.
    let sum: f64 = [
        (-32.0, 4, 1),
        (-1.0, 4, 3),
        (256.0, 10, 1),
        (-64.0, 10, 3),
        (-4.0, 10, 5),
        (-4.0, 10, 7),
        (1.0, 10, 9),
    ].par_iter().map(|(num_const, denom_scalar, denom_const)| {
        num_const * compute_sum(n, *denom_scalar, *denom_const)
    }).sum();

    format!("{:x}", ((sum - sum.floor()) * 16.0).floor() as usize)
}

fn compute_sum(digit: usize, denom_scalar: usize, denom_const: usize) -> f64 {

    // Splitting the sum in a finite and infinite part is slightly more complicated than in the BBP algorithm:
    // 
    // 16^digit \cdot \Sigma_0^{\inf} \frac{1}{2^{10k-6} \cdot (denom_scalar \cdot k + denom_const)}
    // = \Sigma_0^{\inf} \frac{(2^{4 \cdot digit - 10k - 6}}{denom_scalar \cdot k + denom_const}
    // = \Sigma_0^{\inf} \frac{(4^{2 \cdot digit - 5k - 3}}{denom_scalar \cdot k + denom_const}
    // = \Sigma_0^{z} \frac{4^{2 \cdot digit - 5k - 3}}{denom_scalar\cdot k + denom_const} 
    // + \Sigma_0^{z} \frac{4^{2 \cdot digit - 5k - 3}}{denom_scalar\cdot k + denom_const}
    //
    // If z is chosen such the numerator of the first sum is always larger than 0 then we can apply modular exponentiation 
    // to keep the sum small. This is the case for z < (2 \cdot digit - 3) / 5.

    let sum_bound = if 2 * digit < 3 {0} else { (2 * digit - 3) / 5 };  

    let finite_sum = (0..sum_bound).fold(0.0, |sum, k| {
        let denominator = denom_scalar * k + denom_const;
        let summand = (mod_pow(4, 2 * digit - 5 * k - 3, denominator) as f64) / (denominator as f64);
        let scalar = if k % 2 == 0 {1.0} else {-1.0};
        (sum + scalar * summand).fract()
    });

    let infinite_sum: f64 = (sum_bound..).map(|k| {
        let scalar = if k % 2 == 0 {1.0} else {-1.0};
        scalar * (4.0_f64.powi(2 * (digit as i32) - 5 * (k as i32) - 3) as f64) / (denom_scalar * k + denom_const) as f64
    }).take_while(|&summand| 
        summand.abs() > 1e-13_f64 // f64::EPSILON
    ).sum();

    return (finite_sum + infinite_sum).fract();
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils::valid_hex_digit_compute_fn;

    #[test]
    fn test_bellard() {
        valid_hex_digit_compute_fn(bellard);
    }
}

