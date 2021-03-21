import lychrel
import pytest


@pytest.mark.parametrize(
    "number, expected_result",
    [
        (23, 55),
        (32, 55)
    ]
)
def test_reverse_and_add_function(number, expected_result):
    assert lychrel.reverse_and_add(number) == expected_result
