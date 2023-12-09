from math import ceil, floor, sqrt

import string
import unittest

SAMPLE_INPUT_1 = """\
Time:      7  15   30
Distance:  9  40  200
"""

SAMPLE_INPUT_2 = SAMPLE_INPUT_1

def read_line(line:str)-> list[int]:
    _header, data = line.split(':')
    return [ int(i) for i in data.split()]

def f(x:int, c:int)-> int:
    return x*(c-x)

def f_max(c:int) -> int:
    # This is just the first derivative solved for 0
    return c/2

def first_root(time:int, dist:int) -> int:
    return (-time + sqrt( time*time - 4*dist ))/ -2

def second_root(time:int, dist:int) -> int:
    return (-time - sqrt( time*time - 4*dist ))/ -2

def how_man_wins(time:int, current_record:int, best_possible:int) -> int:
    cnt = 0
    for i in range(best_possible, time):
        if f(i,time) > current_record:
            cnt += 1
    if time % 2 == 0:
        return 2*cnt -1
    else:
        return 2*cnt # Why double? because quadratic function is symmetric
                   
def part1(strings: list[str]) -> int:
    result: int = 1
    times = read_line(strings[0])
    dists = read_line(strings[1])
    maxs = [ceil(f_max(c)) for c in times]

    win_counts = [ how_man_wins(times[i], dists[i], best_possible) for i, best_possible in enumerate(maxs)]

    for win in win_counts:
        result *= win
            
    return result


def part2(strings: list[str]) -> int:
    result: int = 0
    stripped_time = strings[0].translate(str.maketrans('', '', string.whitespace))
    stripped_dist = strings[1].translate(str.maketrans('', '', string.whitespace))
    time = read_line(stripped_time)[0]
    dist = read_line(stripped_dist)[0]

    fr = first_root(time,dist)
    sr = second_root(time,dist)

    return len(range(ceil(fr),ceil(sr)))


class TestAoC01(unittest.TestCase):

    def test_part1_max(self):
        self.assertEqual(3.5, f_max(7))
        self.assertEqual(7.5, f_max(15))
        self.assertEqual(15, f_max(30))

    def test_part1_func(self):
        self.assertEqual(12.25, f(3.5,7))
        self.assertEqual(56.25, f(7.5,15))
        self.assertEqual(225, f(15,30))

    def test_part1_how_man_wins(self):
        self.assertEqual(4, how_man_wins(7, 9, ceil(3.5)))
        self.assertEqual(8, how_man_wins(15,40,ceil(7.5)))
        self.assertEqual(9, how_man_wins(30,200,ceil(15)))

    def test_quadratic(self):
        fr = first_root(7,9)
        sr = second_root(7,9)
        self.assertGreater(fr,1)
        self.assertLess(fr,2)
        self.assertGreater(sr,5)
        self.assertLess(sr,6)
        self.assertEqual( 4, 1+len(range(ceil(fr), floor(sr))))

    # @unittest.skip("temp")
    def test_part1_sample_input(self):
        self.assertEqual(288, part1(SAMPLE_INPUT_1.splitlines()))

    # @unittest.skip("temp")
    def test_part1_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part1(lines))

    # @unittest.skip("temp")
    def test_part2_sample_input(self):
        self.assertEqual(71503, part2(SAMPLE_INPUT_2.splitlines()))

    # @unittest.skip("temp")
    def test_part2_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part2(lines))


if __name__ == '__main__':
    unittest.main()
