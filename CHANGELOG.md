# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0] - 2026-09-19

### Breaking
- `fibonacci` takes only `number`. Replace `fibonacci(n, p, q)` with
  `horadam(n, p=p, q=q)`

### Added
- `lucas`: the Lucas numbers
- `horadam`: any recurrence W(n) = p*W(n-1) - q*W(n-2) with initial terms
  W(0) = a and W(1) = b. It covers both kinds of Lucas sequences, such as
  the Pell and Jacobsthal numbers

### Changed
- Development uses [uv](https://docs.astral.sh/uv/): dependency groups in
  `pyproject.toml` and a committed `uv.lock` replace `requirements-dev.txt`
  and `docs/requirements.txt`
- CI lints Rust and Python, runs the tests with uv, builds Windows ARM64
  wheels, and publishes to PyPI through trusted publishing
- Read the Docs builds with `uv sync`
- Documentation rewritten as three pages: overview, reference, and
  development guide
- Faster calls: integers that fit in 64 bits skip the arbitrary-precision
  conversion, which roughly halves the cost of calls on small numbers
- Long computations release the GIL, so calls from several threads run in
  parallel
- Upgraded PyO3 to 0.29, with a declarative module definition, and pinned
  dependency versions
- Minimum supported Python is 3.8

### Fixed
- `kaprekar` keeps the starting digit count, padding with leading zeros, so
  numbers such as 2111 reach 6174 instead of 0
- `kaprekar` raises `ValueError` for a `base` outside 2-256 instead of
  panicking
- `collatz` accepts integers of any size. It used 128-bit integers, so a term
  above 2^128 overflowed and returned a wrong sequence

### Removed
- `polyfill.io` script and MathJax from the documentation

## [0.8.0] - 2025-10-24

### Breaking
- `read_out_loud` renamed to `look_and_say`

### Changed
- Upgraded PyO3 from 0.20 to 0.27 and Rust to the 2024 edition
- Optional arguments are declared with explicit signatures, as PyO3 0.27
  requires
- Docstrings with examples for every function, and expanded documentation
- ruff replaces black and isort

## [0.7.2] - 2023-11-01

Earlier versions have no changelog; see the
[tags](https://github.com/alesanfra/lychrel/tags) and the git history.

[0.9.0]: https://github.com/alesanfra/lychrel/compare/0.8.0...0.9.0
[0.8.0]: https://github.com/alesanfra/lychrel/compare/0.7.2...0.8.0
[0.7.2]: https://github.com/alesanfra/lychrel/releases/tag/0.7.2
