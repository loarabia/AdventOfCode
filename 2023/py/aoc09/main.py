import unittest

SAMPLE_INPUT_1 = """\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"""

SAMPLE_INPUT_2 = SAMPLE_INPUT_1

def read_line(line:str)->list[int]:
    return [ int(num) for num in line.split()]

def pair_it(nums:list[int]) -> tuple[int,int]:
    for i in range(0,len(nums)-1):
        yield (nums[i], nums[i+1])

def diff_readings(nums:list[int])-> list[int]:
    return [ num_pair[1] - num_pair[0] for num_pair in pair_it(nums) ]

def gen_diffs(nums:list[int]) -> list[list[int]]:
    diffs = [nums]
    i = 0
    while not all(num ==0 for num in diffs[i]):
        diffs.append(diff_readings(diffs[i]))
        i += 1
    return diffs

def gen_priors(readings:list[list[int]]) -> int:
    i = len(readings) -1
    while i > 0:
        figure_prior(readings[i], readings[i-1])
        i -= 1
    return readings[0][0]

def figure_prior(lower: list[int], upper:list[int]) -> list[int]:
    upper.insert(0, upper[0]-lower[0])
    return upper

def gen_nexts(readings:list[list[int]]) -> int:
    i = len(readings) -1
    while i > 0:
        figure_next(readings[i], readings[i-1])
        i -= 1
    return readings[0][-1]

def figure_next(lower: list[int], upper:list[int]) -> list[int]:
    upper.append(upper[-1]+lower[-1])
    return upper

def part1(strings: list[str]) -> int:
    result: int = 0
    readings = [read_line(line) for line in strings]
    for reading in readings:
        result += gen_nexts(gen_diffs(reading))
    return result

def part2(strings: list[str]) -> int:
    result: int = 0
    readings = [read_line(line) for line in strings]
    for reading in readings:
        result += gen_priors(gen_diffs(reading))
    return result


class TestAoC01(unittest.TestCase):

    def test_pair_it(self):
        it = pair_it([1,2,3,4])
        self.assertTupleEqual((1,2), next(it))
        self.assertTupleEqual((2,3), next(it))
        self.assertTupleEqual((3,4), next(it))
        self.assertRaises(StopIteration, next, it)

    def test_diff_readings(self):
        diff = diff_readings([1,2,3,4])
        self.assertEqual(len(diff),3)
        self.assertEqual(1, diff[0])
        self.assertEqual(1, diff[1])
        self.assertEqual(1, diff[2])

    def test_gen_diffs(self):
        diffs = gen_diffs([1,2,3,4])
        self.assertEqual(len(diffs), 3)
        self.assertEqual([1,2,3,4], diffs[0])
        self.assertEqual([1,1,1], diffs[1])
        self.assertEqual([0,0], diffs[2])

    # @unittest.skip("temp")
    def test_part1_sample_input(self):
        self.assertEqual(114, part1(SAMPLE_INPUT_1.splitlines()))

    # @unittest.skip("temp")
    def test_part1_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part1(lines))

    # @unittest.skip("temp")
    def test_part2_sample_input(self):
        self.assertEqual(2, part2(SAMPLE_INPUT_2.splitlines()))

    # @unittest.skip("temp")
    def test_part2_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part2(lines))


if __name__ == '__main__':
    unittest.main()
