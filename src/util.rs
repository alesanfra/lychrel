use num_bigint::BigUint;

pub fn sorted_digits(n: &BigUint, base: u32) -> Vec<u8> {
    let mut sorted = n.to_radix_be(base);
    sorted.sort_unstable();
    sorted
}
