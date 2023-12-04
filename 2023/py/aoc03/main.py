from typing import List, Tuple, NamedTuple, Set, Dict
from queue import Queue

import unittest

SAMPLE_INPUT_1 = """\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""

SAMPLE_INPUT_2 = SAMPLE_INPUT_1


class Point(NamedTuple):
    x: int
    y: int


class SchematicNumber(NamedTuple):
    start: Point
    length: int
    value: int
    neighbors: Dict[Point, str]
    # the str is the value of the neighbor such as '.' or '*'.
    # Note: The neighbors dictionary will include the number string itself


# OMG the newlines. Bit me in the rear on this. Very frustrating
def issymbol(c: str):
    return c != '.' and c != '\n' and c != '\r' and not c.isdigit()


# So you don't have to think about it, ALWAYS access a point in the schematic via this getter.
def get(schematic: List[str], x: int, y: int) -> int:
    return schematic[y][x]


def read_schematic_number(schematic: List[str], start: Point) -> SchematicNumber:
    number_string = read_schematic_number_string(schematic[start.y], start.x)
    length = len(number_string)
    value = int(number_string)
    neighbors = read_schematic_number_neighbors(schematic, start, length)
    return SchematicNumber(start, length, value, neighbors)


def read_schematic_number_string(schematic_line: str, start: int) -> str:
    end = start
    while schematic_line[end].isdigit():
        end += 1
    return schematic_line[start:end]


def read_schematic_number_neighbors(schematic: List[str], start: Point, length: int) -> Dict[Point, str]:
    neighbors = {}
    for x in range(start.x-1, start.x+length+1):  # Python's range function isn't inclusive
        for y in range(start.y-1, start.y+2):
            point = Point(x, y)
            if inbounds(schematic, point):
                neighbors[point] = get(schematic, x, y)
    return neighbors


def inbounds(schematic: List[str], point: Point) -> bool:
    min_x, min_y = 0, 0
    max_x, max_y = len(schematic[0]), len(schematic)
    if point.x >= min_x and point.x < max_x and point.y >= min_y and point.y < max_y:
        return True
    return False


def is_symbol_adjacent(number: SchematicNumber) -> bool:
    for n in number.neighbors.values():
        if issymbol(n):
            return True
    return False

# 1st attempt was TOO HIGH! -- I hadn't handled newlines and exlucded them as symbols...


def part1(schematic: List[str]) -> int:
    result: int = 0
    skip: int = 0
    numbers: List[SchematicNumber] = []

    for y, line in enumerate(schematic):
        for x, point in enumerate(line):
            if point.isdigit() and not skip:
                snum = read_schematic_number(schematic, Point(x, y))
                skip = snum.length - 1
                numbers.append(snum)
            elif point.isdigit() and skip:
                skip -= 1
                # Let the iterator advance otherwise for a number like 432,
                # you'll read 432, 32, 2 all as distinct numbers which would be wrong.

    for num in numbers:
        if is_symbol_adjacent(num):
            result += num.value

    return result


def part2(schematic: List[str]) -> int:
    result: int = 0
    skip: int = 0
    numbers: List[SchematicNumber] = []
    for y, line in enumerate(schematic):
        for x, point in enumerate(line):
            if point.isdigit() and not skip:
                snum = read_schematic_number(schematic, Point(x, y))
                skip = snum.length - 1
                numbers.append(snum)
            elif point.isdigit() and skip:
                skip -= 1

    # The Point is the location of a symbol (* or gear in this case)
    # The List is the values of the SchematicNumbers next to it
    links: Dict[Point, List[int]] = {}
    for num in numbers:
        for k, v in num.neighbors.items():
            if v == '*':
                link = links.get(k, [])
                link.append(num.value)
                links[k] = link

    for k, v in links.items():
        if len(v) == 2:
            result += (v[0] * v[1])
            # If it had turned out there were triple linkages or more instead of just double, then
            # this could be turned into an accumulator over all length 2 or great arrays.
        else:
            assert len(v) < 2, "Shouldn't be any longer strings"

    return result


class TestAoC01(unittest.TestCase):

    # @unittest.skip("temp")
    def test_part1_sample_input(self):
        self.assertEqual(4361, part1(SAMPLE_INPUT_1.splitlines()))

    # @unittest.skip("temp")
    def test_part1_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part1(lines))

    # @unittest.skip("temp")
    def test_part2_sample_input(self):
        self.assertEqual(467835, part2(SAMPLE_INPUT_2.splitlines()))

    # @unittest.skip("temp")
    def test_part2_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part2(lines))


if __name__ == '__main__':
    unittest.main()
