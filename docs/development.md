# Development

You need [uv](https://docs.astral.sh/uv/) and a Rust toolchain from
[rustup](https://rustup.rs/).

## Setup

```console
git clone https://github.com/alesanfra/lychrel.git
cd lychrel
uv venv -p 3.14
uv sync --frozen
uv run --no-sync pre-commit install
uv run maturin develop --uv
```

`maturin develop` compiles the Rust extension into the virtual environment.
Run it again after every change to `src/lib.rs`.

!!! warning "Use `--no-sync` after building"
    `uv run` re-syncs the environment by default, which replaces the module
    you just built. Run everything else as `uv run --no-sync ...`.

## Layout

| Path | Contents |
| --- | --- |
| `src/lib.rs` | Every public function, with its docstring |
| `lychrel/__init__.py` | Re-exports the compiled module |
| `lychrel/py.py` | Pure-Python versions used by the benchmarks |
| `tests/` | pytest suite |
| `docs/` | This site |

## Tests

```console
uv run --no-sync pytest
uv run --no-sync pytest --cov=lychrel
```

Benchmarks compare the Rust functions with `lychrel.py` and are skipped by
default. Build in release mode first, since a debug build can be slower than
pure Python:

```console
uv run maturin develop --uv --release
uv run --no-sync pytest -m benchmark
```

## Linting

```console
uv run --no-sync pre-commit run -a
```

This runs `cargo fmt`, `cargo clippy`, `ruff format`, and `ruff check`. CI
runs the same checks.

## Documentation

```console
uv sync --frozen --group docs
uv run --no-sync mkdocs serve
```

Read the Docs builds the site with `uv sync` from `uv.lock`.

## Pull requests

Add tests for new behavior, update the docs and `CHANGELOG.md`, and use
[Conventional Commits](https://www.conventionalcommits.org/) messages
(`feat: ...`, `fix: ...`).

## Releasing

1. Bump the version in `Cargo.toml`. `pyproject.toml` reads it from there.
2. Add a `CHANGELOG.md` entry.
3. Tag the commit with the bare version (`0.9.0`, no `v` prefix) and push
   the tag.

`.github/workflows/ci.yml` builds the wheels and publishes them to PyPI
through trusted publishing.
