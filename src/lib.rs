use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

const BASE: u32 = 10;
const MAX_ITERATIONS: usize = 10000;

/// Find the first palindrome produced by the reverse-and-add routine.
///
/// This function implements the reverse-and-add algorithm used to test for Lychrel numbers.
/// Starting with a number, it repeatedly reverses its digits and adds the result to the
/// original number, checking if the result is a palindrome at each step.
///
/// # Arguments
///
/// * `number` - The starting number to test (any non-negative integer)
/// * `max_iterations` - Maximum number of iterations to try before giving up (default: 10000)
///
/// # Returns
///
/// Returns a tuple `(palindrome, iterations)` where:
/// * `palindrome` - The first palindrome found in the sequence
/// * `iterations` - The number of iterations needed to reach the palindrome (0 if input is already a palindrome)
///
/// # Errors
///
/// Returns a `ValueError` if no palindrome is found within `max_iterations` steps,
/// suggesting the number might be a Lychrel candidate.
///
/// # Examples
///
/// ```python
/// import lychrel
///
/// # 89 becomes a palindrome after 24 iterations
/// palindrome, iterations = lychrel.find_lychrel_palindrome(89)
/// assert palindrome == 8813200023188
/// assert iterations == 24
///
/// # 10 is already close to a palindrome
/// palindrome, iterations = lychrel.find_lychrel_palindrome(10)
/// assert palindrome == 11
/// assert iterations == 1
///
/// # 196 is a suspected Lychrel number
/// try:
///     lychrel.find_lychrel_palindrome(196, max_iterations=100)
/// except ValueError:
///     print("No palindrome found - likely a Lychrel candidate")
/// ```
#[pyfunction]
#[pyo3(signature = (number, max_iterations=None))]
fn find_lychrel_palindrome(
    number: BigUint,
    max_iterations: Option<usize>,
) -> PyResult<(BigUint, usize)> {
    let max_iterations = max_iterations.unwrap_or(MAX_ITERATIONS);
    let mut next: BigUint = number;

    for iterations in 0..max_iterations {
        let base10_representation = next.to_radix_le(BASE);

        // Check whether the decimal representation is palindrome
        if base10_representation
            .iter()
            .eq(base10_representation.iter().rev())
        {
            return Ok((next, iterations));
        }

        // Reverse and add
        next += BigUint::from_radix_be(&base10_representation, BASE).unwrap();
    }

    Err(PyValueError::new_err("Maximum iterations reached"))
}

/// Check whether a number is a potential Lychrel number.
///
/// A Lychrel number is a natural number that never forms a palindrome through the
/// iterative process of reversing its digits and adding the resulting number to the original.
/// No Lychrel numbers have been proven to exist in base 10, but 196 is the smallest candidate.
///
/// # Arguments
///
/// * `number` - The number to test for Lychrel candidacy
/// * `max_iterations` - Maximum iterations to try (default: 10000). If no palindrome is
///   found within this limit, the number is considered a Lychrel candidate.
///
/// # Returns
///
/// * `true` - If the number appears to be a Lychrel candidate (no palindrome found)
/// * `false` - If a palindrome is found within the iteration limit
///
/// # Examples
///
/// ```python
/// import lychrel
///
/// # 196 is the most famous Lychrel candidate
/// assert lychrel.is_lychrel_candidate(196) == True
///
/// # 197 eventually forms a palindrome
/// assert lychrel.is_lychrel_candidate(197) == False
///
/// # Most numbers form palindromes quickly
/// assert lychrel.is_lychrel_candidate(89) == False
///
/// # You can adjust the iteration limit
/// # This might return True if the number needs more iterations
/// lychrel.is_lychrel_candidate(197, max_iterations=5)
/// ```
///
/// # Note
///
/// This function returns `true` for suspected Lychrel candidates, but cannot prove
/// a number is truly a Lychrel number (which would require infinite iterations).
#[pyfunction]
#[pyo3(signature = (number, max_iterations=None))]
fn is_lychrel_candidate(number: BigUint, max_iterations: Option<usize>) -> bool {
    find_lychrel_palindrome(number, max_iterations).is_err()
}

/// Compute the nth term of a generalized Fibonacci sequence (Lucas sequence).
///
/// This function implements Lucas sequences, which are generalizations of the Fibonacci sequence.
/// The sequence is defined by the recurrence relation:
///
///     F(n) = p * F(n-1) - q * F(n-2)
///
/// with initial conditions F(0) = 0 and F(1) = 1.
///
/// # Arguments
///
/// * `number` - The position in the sequence (n >= 0)
/// * `p` - The first parameter of the recurrence relation (default: 1)
/// * `q` - The second parameter of the recurrence relation (default: -1)
///
/// # Returns
///
/// The nth term of the sequence as a BigInt (supports arbitrarily large numbers).
///
/// # Special Cases
///
/// Different values of p and q produce different famous sequences:
///
/// * **Fibonacci** (p=1, q=-1): F(n) = F(n-1) + F(n-2)
///   - Sequence: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
///
/// * **Pell** (p=2, q=-1): F(n) = 2*F(n-1) + F(n-2)
///   - Sequence: 0, 1, 2, 5, 12, 29, 70, 169, 408, 985, ...
///
/// * **Jacobsthal** (p=1, q=-2): F(n) = F(n-1) + 2*F(n-2)
///   - Sequence: 0, 1, 1, 3, 5, 11, 21, 43, 85, 171, ...
///
/// # Examples
///
/// ```python
/// import lychrel
///
/// # Standard Fibonacci sequence
/// assert lychrel.fibonacci(10) == 55
/// assert lychrel.fibonacci(20) == 6765
///
/// # Pell numbers
/// assert lychrel.fibonacci(10, p=2, q=-1) == 2378
///
/// # Jacobsthal numbers
/// assert lychrel.fibonacci(10, p=1, q=-2) == 341
///
/// # Works with very large numbers
/// big_fib = lychrel.fibonacci(1000)
/// print(f"Fibonacci(1000) has {len(str(big_fib))} digits")
/// # Output: Fibonacci(1000) has 209 digits
/// ```
///
/// # Performance
///
/// The function uses an iterative algorithm with O(n) time complexity and O(1) space
/// complexity. Thanks to Rust's BigInt, it handles arbitrarily large results efficiently.
#[pyfunction]
#[pyo3(signature = (number, p=None, q=None))]
fn fibonacci(number: usize, p: Option<isize>, q: Option<isize>) -> BigInt {
    if number <= 1 {
        number.to_bigint().unwrap()
    } else {
        let lucas_p = p.unwrap_or(1);
        let lucas_q = q.unwrap_or(-1);

        let mut previous = BigInt::zero();
        let mut current = BigInt::one();

        for _ in 1..number {
            let next_previous = current.clone();
            let next_current = (current * lucas_p) - (previous * lucas_q);
            previous = next_previous;
            current = next_current;
        }

        current
    }
}

/// Generate the "read out loud" representation of a number (Look-and-Say sequence).
///
/// This function implements the "Look-and-Say" or "Morris Number" sequence algorithm.
/// It reads the digits of a number out loud by counting consecutive occurrences of each digit.
///
/// # Arguments
///
/// * `number` - The number to read out loud (any non-negative integer)
///
/// # Returns
///
/// A new number representing the "read out loud" version of the input.
///
/// # Errors
///
/// Returns a `ValueError` if the resulting number cannot be represented in base 10
/// (extremely rare, only with malformed internal representations).
///
/// # Examples
///
/// ```python
/// import lychrel
///
/// # Single digit: "one 1"
/// assert lychrel.look_and_say(1) == 11
///
/// # Two different digits: "one 1, one 2"
/// assert lychrel.look_and_say(12) == 1112
///
/// # Multiple same digits: "one 3, one 2, two 1s"
/// assert lychrel.look_and_say(3211) == 131221
///
/// # Complex example: "one 2, three 3s, two 5s"
/// assert lychrel.look_and_say(2333355) == 124325
///
/// # Generate the Look-and-Say sequence
/// n = 1
/// for _ in range(5):
///     print(n)
///     n = lychrel.look_and_say(n)
/// # Output: 1, 11, 21, 1211, 111221
/// ```
///
/// # Mathematical Properties
///
/// When starting from 1, the Look-and-Say sequence has interesting properties:
/// * Never contains digits greater than 3
/// * Each term is about 30% longer than the previous (Conway's constant ≈ 1.303577)
/// * Never contains the substring "333"
/// * Related to Conway's cosmological theorem
#[pyfunction]
fn look_and_say(number: BigUint) -> PyResult<BigUint> {
    let mut current_digit: u8 = 0;
    let mut count: u8 = 0;
    let mut result: Vec<u8> = Vec::new();

    for i in number.to_radix_be(BASE) {
        if count == 0 {
            current_digit = i;
        }

        if i == current_digit {
            count += 1;
        } else {
            result.push(count);
            result.push(current_digit);
            current_digit = i;
            count = 1;
        }
    }

    result.push(count);
    result.push(current_digit);

    BigUint::from_radix_be(&result, BASE).ok_or_else(|| {
        PyValueError::new_err(format!("Unable to read out loud the number {}", number))
    })
}

#[inline(always)]
fn sorted_digits(n: &BigUint, base: u32) -> Vec<u8> {
    let mut sorted = n.to_radix_be(base);
    sorted.sort_unstable();
    sorted
}

/// Apply Kaprekar's routine to find the Kaprekar constant.
///
/// Kaprekar's routine is an algorithm that takes a natural number, sorts its digits
/// in descending and ascending order to form two new numbers, and subtracts the smaller
/// from the larger. This process is repeated until a fixed point is reached.
///
/// For 4-digit numbers in base 10, the routine converges to **6174** (Kaprekar's constant).
///
/// # Arguments
///
/// * `number` - The starting number
/// * `base` - The number base to use (default: 10)
/// * `max_iterations` - Maximum iterations before giving up (default: 10000)
///
/// # Returns
///
/// The Kaprekar constant (fixed point) for the given number and base.
///
/// # Errors
///
/// Returns a `ValueError` if:
/// * The maximum number of iterations is reached without finding a fixed point
/// * The number is not valid in the specified base
///
/// # Algorithm
///
/// 1. Sort the digits in descending order → largest number
/// 2. Sort the digits in ascending order → smallest number
/// 3. Subtract smallest from largest → result
/// 4. If result equals previous value, return it (fixed point found)
/// 5. Otherwise, repeat with result
///
/// # Examples
///
/// ```python
/// import lychrel
///
/// # All 4-digit numbers converge to 6174
/// assert lychrel.kaprekar(1234) == 6174
/// assert lychrel.kaprekar(9876) == 6174
/// assert lychrel.kaprekar(4680) == 6174
///
/// # Example trace for 3524:
/// # 5432 - 2345 = 3087
/// # 8730 - 0378 = 8352
/// # 8532 - 2358 = 6174
/// # 7641 - 1467 = 6174 (fixed point!)
/// assert lychrel.kaprekar(3524) == 6174
///
/// # Specify different base (advanced)
/// result = lychrel.kaprekar(1234, base=10)
///
/// # Control iteration limit
/// result = lychrel.kaprekar(1234, max_iterations=100)
/// ```
///
/// # Kaprekar Constants
///
/// Different digit lengths have different constants:
/// * 3 digits: 495
/// * 4 digits: 6174
/// * 5 digits: Multiple cycles possible
/// * 6 digits: Multiple constants (e.g., 549945, 631764)
#[pyfunction]
#[pyo3(signature = (number, base=None, max_iterations=None))]
fn kaprekar(
    number: BigUint,
    base: Option<u32>,
    max_iterations: Option<usize>,
) -> PyResult<BigUint> {
    let _base = base.unwrap_or(BASE);
    let _max_iterations = max_iterations.unwrap_or(MAX_ITERATIONS);
    let mut previous = number;

    for _ in 0.._max_iterations {
        let sorted = sorted_digits(&previous, _base);
        let first = BigUint::from_radix_le(&sorted, _base)
            .ok_or_else(|| PyValueError::new_err("Not a decimal number"))?;
        let second = BigUint::from_radix_be(&sorted, _base)
            .ok_or_else(|| PyValueError::new_err("Not a decimal number"))?;
        let result = first - second;

        if result == previous {
            return Ok(result);
        }

        previous = result;
    }

    Err(PyValueError::new_err("Maximum iteration reached"))
}

/// Generate the Collatz sequence (3n+1 problem) for a given starting number.
///
/// The Collatz conjecture is one of the most famous unsolved problems in mathematics.
/// Starting with any positive integer n, the sequence is defined as:
/// * If n is even: n → n/2
/// * If n is odd: n → 3n+1
///
/// The conjecture states that this sequence always reaches 1, regardless of the starting number.
/// While extensively tested (verified up to 2^68), no mathematical proof exists.
///
/// # Arguments
///
/// * `start` - The starting positive integer (must be > 0)
///
/// # Returns
///
/// A vector containing the complete Collatz sequence from `start` to 1 (inclusive).
///
/// # Errors
///
/// Returns a `ValueError` if `start` is 0 or negative.
///
/// # Examples
///
/// ```python
/// import lychrel
///
/// # Simple example: 5 → 16 → 8 → 4 → 2 → 1
/// sequence = lychrel.collatz(5)
/// assert sequence == [5, 16, 8, 4, 2, 1]
///
/// # Longer sequence
/// sequence = lychrel.collatz(27)
/// print(f"Length: {len(sequence)}")  # 111 steps!
/// print(f"Maximum value: {max(sequence)}")  # 9232
///
/// # Find stopping time (steps to reach 1)
/// stopping_time = len(lychrel.collatz(27)) - 1
/// print(f"Stopping time: {stopping_time}")  # 110
///
/// # Iterate through the sequence
/// for i, n in enumerate(lychrel.collatz(10)):
///     print(f"Step {i}: {n}")
///
/// # Error handling
/// try:
///     lychrel.collatz(0)  # Invalid!
/// except ValueError as e:
///     print(f"Error: {e}")
/// ```
///
/// # Interesting Facts
///
/// * No counterexample has ever been found
/// * Some numbers take surprisingly long paths to reach 1
/// * The number 27 reaches a maximum of 9,232 before descending
/// * 97 requires 118 steps to reach 1
/// * The sequence is also called the "hailstone sequence" due to its up-and-down nature
///
/// # Performance
///
/// The function generates the complete sequence in memory. For very large starting numbers
/// or numbers with exceptionally long sequences, memory usage should be considered.
#[pyfunction]
fn collatz(start: u128) -> PyResult<Vec<u128>> {
    if start == 0 {
        Err(PyValueError::new_err("Start number must be > 0"))
    } else {
        let mut result: Vec<u128> = vec![start];
        let mut current: u128 = start;

        while current != 1 {
            current = if current % 2 != 0 {
                current * 3 + 1
            } else {
                current / 2
            };
            result.push(current);
        }

        Ok(result)
    }
}

/// A collection of functions to play with Lychrel numbers and other funny mathematical problems
#[pymodule]
fn lychrel(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_function(wrap_pyfunction!(is_lychrel_candidate, module)?)?;
    module.add_function(wrap_pyfunction!(find_lychrel_palindrome, module)?)?;
    module.add_function(wrap_pyfunction!(fibonacci, module)?)?;
    module.add_function(wrap_pyfunction!(look_and_say, module)?)?;
    module.add_function(wrap_pyfunction!(kaprekar, module)?)?;
    module.add_function(wrap_pyfunction!(collatz, module)?)?;

    Ok(())
}
