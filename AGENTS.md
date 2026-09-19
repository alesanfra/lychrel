# AGENTS.md

Working notes for coding agents (and humans) touching this repository.

## What this project is

`lychrel` is a Python extension module written in Rust with
[PyO3](https://pyo3.rs/) and built by [maturin](https://www.maturin.rs/). It
implements a handful of recreational-math routines: `find_lychrel_palindrome`,
`is_lychrel_candidate`, `fibonacci`, `lucas`, `horadam` (the general
second-order recurrence behind both), `look_and_say`,
`kaprekar`, and `collatz`.

The package uses maturin's mixed layout: the compiled module is installed as
`lychrel.lychrel` and re-exported by `lychrel/__init__.py`. `lychrel/py.py`
holds pure-Python reference implementations used only by the benchmarks.

## Rust conventions

- The module is declared with `#[pymodule] mod lychrel` at the end of
  `src/lib.rs`; add new functions to its `#[pymodule_export] use` list.
- Take and return integers as `Natural` (non-negative) or `Integer` (signed),
  not `BigUint`/`BigInt`. Under abi3, PyO3 converts big integers through
  `int.to_bytes`/`int.from_bytes`; the wrappers use native 64-bit
  conversions when the value fits, which halves the cost of small calls.
- Keep the computation in a plain Rust function and call it inside
  `py.detach(...)`, so that threads run in parallel.

## Layout

| Path | Contents |
| --- | --- |
| `src/lib.rs` | PyO3 module: every public function and its docstring |
| `lychrel/__init__.py` | Re-exports the compiled module |
| `lychrel/py.py` | Pure-Python implementations for benchmarks |
| `tests/` | pytest suite |
| `docs/` | MkDocs site published on Read the Docs |

## Docs

Three pages: `index.md` (overview), `reference.md` (one section per
function), `development.md`. Every example must match the output of a
freshly built module; check them instead of writing them from memory.
Keep the text short, in American English, with no performance claims that
have not been measured.

## Environment

Requires [uv](https://docs.astral.sh/uv/) and a Rust toolchain.

```bash
uv venv -p 3.14                # once
uv sync --frozen               # dev dependencies
uv sync --frozen --all-groups  # add the docs dependencies too
uv run maturin develop --uv    # compile the extension into the venv
```

**`uv run` re-syncs the project by default and overwrites the module that
`maturin develop` just built.** Always run tests and scripts as:

```bash
uv run --no-sync pytest
```

Re-run `uv run maturin develop --uv` after every change to a `.rs` file; the
Rust code is not rebuilt automatically by pytest.

## Checks to run before proposing a change

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
uv run maturin develop --uv
uv run --no-sync pytest
```

CI runs the same four, plus `ruff check .` and `ruff format --check .` for
the Python files. `pre-commit run -a` covers the formatters and linters
locally.

Benchmarks are deselected by default. Run them against a release build,
since debug builds are slower than the pure-Python code:

```bash
uv run maturin develop --uv --release
uv run --no-sync pytest -m benchmark
```

## Building the docs

```bash
uv sync --frozen --group docs
uv run --no-sync mkdocs serve
```

Read the Docs builds with `uv sync` against `uv.lock` and compiles the
extension with the `latest` Rust toolchain.

## Conventions

- Commits follow [Conventional Commits](https://www.conventionalcommits.org/).
- Rust: `cargo fmt` defaults, no `unwrap()` on anything reachable from Python
  input (a panic surfaces as `PanicException`).
- Python: ruff with a 79-column limit.
- Comments explain why, not what.

## Release

Version lives in `Cargo.toml` and is re-exported as `lychrel.__version__`;
`pyproject.toml` takes it from there. To release: bump the version, update
`CHANGELOG.md`, tag with the bare version (`0.9.0`), and let
`.github/workflows/ci.yml` build the wheels,
attest them in the `release` job, and upload them in the `publish` job. The
upload uses PyPI trusted publishing, bound to this workflow file and the
`pypi` environment name: renaming either breaks publishing until the
publisher is updated on PyPI.
