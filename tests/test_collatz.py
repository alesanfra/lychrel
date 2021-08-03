import lychrel
import pytest


@pytest.mark.parametrize(
    "n, expected",
    [
        (5, [5, 16, 8, 4, 2, 1]),
    ],
)
def test_collatz(n, expected):
    assert list(lychrel.collatz(n)) == expected


def test_collatz_zero():
    with pytest.raises(ValueError):
        lychrel.collatz(0)
