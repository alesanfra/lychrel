import pytest

import lychrel


@pytest.mark.parametrize(
    "number, expected_output",
    [(1, 11), (12, 1112), (3211, 131221), (2333355, 124325)],
)
def test_look_and_say(number, expected_output):
    assert lychrel.look_and_say(number) == expected_output
