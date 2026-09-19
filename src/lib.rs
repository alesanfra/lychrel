use num_bigint::{BigInt, BigUint};
use num_traits::{One, ToPrimitive, Zero};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyInt, PyList};

const BASE: u32 = 10;
const MAX_ITERATIONS: usize = 10000;

// Under the stable ABI, PyO3 converts arbitrary-precision integers through
// `int.to_bytes` and `int.from_bytes`, which costs more than most of the
// computations here. `Natural` and `Integer` convert values that fit in 64
// bits natively and use BigUint and BigInt only for larger ones.

/// A non-negative Python int.
struct Natural(BigUint);

impl FromPyObject<'_, '_> for Natural {
    type Error = PyErr;

    fn extract(ob: Borrowed<'_, '_, PyAny>) -> PyResult<Self> {
        match ob.extract::<u64>() {
            Ok(n) => Ok(Natural(n.into())),
            Err(_) => ob.extract::<BigUint>().map(Natural),
        }
    }
}

impl<'py> IntoPyObject<'py> for Natural {
    type Target = PyInt;
    type Output = Bound<'py, PyInt>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        match self.0.to_u64() {
            Some(n) => Ok(n.into_pyobject(py)?),
            None => self.0.into_pyobject(py),
        }
    }
}

/// A Python int of either sign.
struct Integer(BigInt);

impl FromPyObject<'_, '_> for Integer {
    type Error = PyErr;

    fn extract(ob: Borrowed<'_, '_, PyAny>) -> PyResult<Self> {
        match ob.extract::<i64>() {
            Ok(n) => Ok(Integer(n.into())),
            Err(_) => ob.extract::<BigInt>().map(Integer),
        }
    }
}

impl<'py> IntoPyObject<'py> for Integer {
    type Target = PyInt;
    type Output = Bound<'py, PyInt>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        match self.0.to_i64() {
            Some(n) => Ok(n.into_pyobject(py)?),
            None => self.0.into_pyobject(py),
        }
    }
}

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
    py: Python<'_>,
    number: Natural,
    max_iterations: Option<usize>,
) -> PyResult<(Natural, usize)> {
    let max_iterations = max_iterations.unwrap_or(MAX_ITERATIONS);
    py.detach(|| first_palindrome(number.0, max_iterations))
        .map(|(palindrome, iterations)| (Natural(palindrome), iterations))
        .ok_or_else(|| PyValueError::new_err("Maximum iterations reached"))
}

/// Run reverse-and-add until a palindrome, checking at most `max_iterations` values.
fn first_palindrome(mut next: BigUint, max_iterations: usize) -> Option<(BigUint, usize)> {
    for iterations in 0..max_iterations {
        let digits = next.to_radix_le(BASE);

        if digits.iter().eq(digits.iter().rev()) {
            return Some((next, iterations));
        }

        // Reading the little-endian digits as big-endian reverses the number.
        next += BigUint::from_radix_be(&digits, BASE)?;
    }

    None
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
fn is_lychrel_candidate(py: Python<'_>, number: Natural, max_iterations: Option<usize>) -> bool {
    let max_iterations = max_iterations.unwrap_or(MAX_ITERATIONS);
    py.detach(|| first_palindrome(number.0, max_iterations))
        .is_none()
}

/// Term `number` of the sequence W(0) = a, W(1) = b, W(n) = p*W(n-1) - q*W(n-2).
fn second_order_term(number: usize, a: BigInt, b: BigInt, p: isize, q: isize) -> BigInt {
    if number == 0 {
        return a;
    }

    let mut previous = a;
    let mut current = b;

    for _ in 1..number {
        let next = &current * p - &previous * q;
        previous = std::mem::replace(&mut current, next);
    }

    current
}

/// Compute the nth term of a Horadam sequence.
///
/// A Horadam sequence is defined by two initial values and the recurrence
///
///     W(0) = a,  W(1) = b,  W(n) = p * W(n-1) - q * W(n-2)
///
/// With a=0, b=1 it is the Lucas sequence of the first kind U(p, q), which
/// includes the Fibonacci (p=1, q=-1), Pell (p=2, q=-1), and Jacobsthal
/// (p=1, q=-2) numbers. With a=2, b=p it is the Lucas sequence of the second
/// kind V(p, q), which includes the Lucas numbers (p=1, q=-1).
///
/// # Arguments
///
/// * `number` - The position in the sequence (n >= 0)
/// * `a` - The term W(0) (default: 0)
/// * `b` - The term W(1) (default: 1)
/// * `p` - The first parameter of the recurrence (default: 1)
/// * `q` - The second parameter of the recurrence (default: -1)
///
/// # Examples
///
/// ```python
/// import lychrel
///
/// # Defaults give the Fibonacci numbers
/// assert lychrel.horadam(10) == 55
///
/// # Pell numbers
/// assert lychrel.horadam(10, p=2, q=-1) == 2378
///
/// # Same recurrence, different start: 3, 4, 7, 11, 18, ...
/// assert lychrel.horadam(4, a=3, b=4) == 18
/// ```
#[pyfunction]
#[pyo3(signature = (number, a=None, b=None, p=None, q=None))]
fn horadam(
    py: Python<'_>,
    number: usize,
    a: Option<Integer>,
    b: Option<Integer>,
    p: Option<isize>,
    q: Option<isize>,
) -> Integer {
    let a = a.map_or_else(BigInt::zero, |a| a.0);
    let b = b.map_or_else(BigInt::one, |b| b.0);
    let (p, q) = (p.unwrap_or(1), q.unwrap_or(-1));
    Integer(py.detach(|| second_order_term(number, a, b, p, q)))
}

/// Compute the nth Fibonacci number.
///
/// F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2). For other values of p and q
/// (Pell, Jacobsthal, ...) use `horadam`.
///
/// # Examples
///
/// ```python
/// import lychrel
///
/// assert lychrel.fibonacci(10) == 55
/// ```
#[pyfunction]
fn fibonacci(py: Python<'_>, number: usize) -> Integer {
    Integer(py.detach(|| second_order_term(number, BigInt::zero(), BigInt::one(), 1, -1)))
}

/// Compute the nth Lucas number.
///
/// L(0) = 2, L(1) = 1, L(n) = L(n-1) + L(n-2). For other values of p and q
/// (Pell-Lucas, ...) use `horadam`.
///
/// # Examples
///
/// ```python
/// import lychrel
///
/// assert lychrel.lucas(10) == 123
/// ```
#[pyfunction]
fn lucas(py: Python<'_>, number: usize) -> Integer {
    Integer(py.detach(|| second_order_term(number, BigInt::from(2), BigInt::one(), 1, -1)))
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
fn look_and_say(number: Natural) -> PyResult<Natural> {
    let number = number.0;
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

    BigUint::from_radix_be(&result, BASE)
        .map(Natural)
        .ok_or_else(|| {
            PyValueError::new_err(format!("Unable to read out loud the number {}", number))
        })
}

#[inline(always)]
fn sorted_digits(n: &BigUint, base: u32, width: usize) -> Vec<u8> {
    let mut sorted = n.to_radix_be(base);
    // Keep the starting width: 999 from a four-digit number is 0999.
    sorted.resize(sorted.len().max(width), 0);
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
/// * `base` is not between 2 and 256
///
/// Every intermediate value keeps the digit count of `number`, padding with
/// leading zeros: 2111 gives 2111 - 1112 = 999, which continues as 9990 - 0999.
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
    py: Python<'_>,
    number: Natural,
    base: Option<u32>,
    max_iterations: Option<usize>,
) -> PyResult<Natural> {
    let number = number.0;
    let base = base.unwrap_or(BASE);
    if !(2..=256).contains(&base) {
        return Err(PyValueError::new_err(format!(
            "base must be between 2 and 256, got {base}"
        )));
    }
    let max_iterations = max_iterations.unwrap_or(MAX_ITERATIONS);
    py.detach(|| kaprekar_fixed_point(number, base, max_iterations))
        .map(Natural)
        .ok_or_else(|| PyValueError::new_err("Maximum iteration reached"))
}

/// Iterate Kaprekar's routine until a fixed point, for at most `max_iterations` steps.
fn kaprekar_fixed_point(number: BigUint, base: u32, max_iterations: usize) -> Option<BigUint> {
    let width = number.to_radix_be(base).len();
    let mut previous = number;

    for _ in 0..max_iterations {
        let sorted = sorted_digits(&previous, base, width);
        let largest = BigUint::from_radix_le(&sorted, base)?;
        let smallest = BigUint::from_radix_be(&sorted, base)?;
        let result = largest - smallest;

        if result == previous {
            return Some(result);
        }

        previous = result;
    }

    None
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
/// Returns a `ValueError` if `start` is 0.
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
/// print(f"Length: {len(sequence)}")  # 112
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
fn collatz(py: Python<'_>, start: Natural) -> PyResult<Bound<'_, PyList>> {
    let start = start.0;
    if start.is_zero() {
        return Err(PyValueError::new_err("Start number must be > 0"));
    }

    let result = PyList::empty(py);
    result.append(Natural(start.clone()))?;
    let mut current = start;

    // Most terms fit in 128 bits, where arithmetic is much cheaper than with
    // BigUint. Fall back to BigUint only for the terms that do not.
    'outer: loop {
        if let Some(mut n) = current.to_u128() {
            while n != 1 {
                if n.is_multiple_of(2) {
                    n /= 2;
                } else if let Some(next) = n.checked_mul(3).and_then(|m| m.checked_add(1)) {
                    n = next;
                } else {
                    current = BigUint::from(n) * 3u32 + 1u32;
                    result.append(Natural(current.clone()))?;
                    continue 'outer;
                }
                match u64::try_from(n) {
                    Ok(small) => result.append(small)?,
                    Err(_) => result.append(n)?,
                }
            }
            return Ok(result);
        }

        current = if current.bit(0) {
            current * 3u32 + 1u32
        } else {
            current >> 1
        };
        result.append(Natural(current.clone()))?;
    }
}

/// A collection of functions to play with Lychrel numbers and other funny mathematical problems
#[pymodule]
mod lychrel {
    use pyo3::prelude::*;

    #[pymodule_export]
    use super::{
        collatz, fibonacci, find_lychrel_palindrome, horadam, is_lychrel_candidate, kaprekar,
        look_and_say, lucas,
    };

    #[pymodule_init]
    fn init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add("__version__", env!("CARGO_PKG_VERSION"))
    }
}
