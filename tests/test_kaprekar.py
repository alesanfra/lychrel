import pytest

import lychrel


@pytest.mark.parametrize(
    "n, expected",
    [
        (1234, 6174),
        (9876, 6174),
        (4680, 6174),
        # Differences below 1000 keep four digits: 999 is read as 0999.
        (2111, 6174),
        (1000, 6174),
        (9998, 6174),
        (123, 495),
        (100, 495),
        (1111, 0),
    ],
)
def test_kaprekar(n, expected):
    assert lychrel.kaprekar(n) == expected


def test_kaprekar_every_four_digit_number_reaches_6174():
    non_repdigits = (n for n in range(1000, 10000) if len(set(str(n))) > 1)
    assert {lychrel.kaprekar(n) for n in non_repdigits} == {6174}


def test_kaprekar_base():
    # 1011 -> 1110 - 0111 = 0111, a fixed point.
    assert lychrel.kaprekar(0b1011, base=2) == 0b0111


@pytest.mark.parametrize("base", [0, 1, 257])
def test_kaprekar_invalid_base(base):
    with pytest.raises(ValueError, match="base must be between 2 and 256"):
        lychrel.kaprekar(1234, base=base)


def test_kaprekar_cycle():
    with pytest.raises(ValueError, match="Maximum iteration reached"):
        lychrel.kaprekar(12345)
