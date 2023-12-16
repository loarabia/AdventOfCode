import unittest

SAMPLE_INPUT_1 = """\
"""

SAMPLE_INPUT_2 = """\
"""


def part1(strings: list[str]) -> int:
    result: int = 0
    return result


def part2(strings: list[str]) -> int:
    result: int = 0
    return result


class TestAoC01(unittest.TestCase):

    @unittest.skip("temp")
    def test_part1_sample_input(self):
        self.assertEqual(0, part1(SAMPLE_INPUT_1.splitlines()))

    @unittest.skip("temp")
    def test_part1_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part1(lines))

    @unittest.skip("temp")
    def test_part2_sample_input(self):
        self.assertEqual(0, part2(SAMPLE_INPUT_2.splitlines()))

    @unittest.skip("temp")
    def test_part2_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part2(lines))


if __name__ == '__main__':
    unittest.main()
