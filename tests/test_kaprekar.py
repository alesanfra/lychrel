import pytest

import lychrel


@pytest.mark.parametrize(
    "n, expected",
    [
        (1234, 6174),
        (9876, 6174),
        (4680, 6174),
    ],
)
def test_kaprekar(n, expected):
    assert lychrel.kaprekar(n) == expected
