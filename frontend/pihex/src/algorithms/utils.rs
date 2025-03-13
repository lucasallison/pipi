
// See: https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
pub fn mod_pow(mut base: usize, mut exp: usize, modulus: usize) -> usize {

    if modulus == 1 { return 0 }

    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

const PI_HEX_100: &str = "243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be5466cf34e90c6cc0ac";

#[allow(dead_code)] // This function is intended to be used in the test module.
pub fn valid_hex_digit_compute_fn(f: fn(usize) -> String) {

    let mut digits: Vec::<(String, usize)> = PI_HEX_100
        .chars()
        .enumerate()
        .map(|(i, c)| (c.to_string(), i))
        .collect();

    digits.extend([
        ("4", 1000), ("c", 5000), ("8", 10000), ("9", 50000),
        ("3", 100000), ("6", 1000000), ("e", 5000000), ("7", 10000000)]
        .iter()
        .map(|(c, i)| (c.to_string(), *i))
    );    

    for (d_correct, i) in digits {

        let d_computed = f(i); 

        assert_eq!(d_computed, d_correct, "Invalid hex digit at index {}: Computed {}, but expected {}", i, d_computed, d_correct);
    }
}