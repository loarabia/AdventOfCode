from itertools import combinations
from typing import NamedTuple

import unittest

SAMPLE_INPUT_1 = """\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"""

SAMPLE_INPUT_2 = SAMPLE_INPUT_1

class Loc(NamedTuple):
    x:int
    y:int

def find_galaxies(strings:list[list[str]]) -> set[Loc]:
    locs = set()
    for y, row in enumerate(strings):
        for x, value in enumerate(row):
            if value == '#':
                locs.add(Loc(x,y))
    return locs

def find_all_empty_rows(width:int, galaxies:set[Loc]) -> list[int]:
    empty_rows = [True for _ in range(0,width)]
    for galaxy in galaxies:
        empty_rows[galaxy.y] = False
    return [i for i in range(0,len(empty_rows)) if empty_rows[i] is True]

def find_all_empty_columns(height:int, galaxies:set[Loc])-> list[int]:
    empty_cols = [True for _ in range(0,height)]
    for galaxy in galaxies:
        empty_cols[galaxy.x] = False
    return [i for i in range(0,len(empty_cols)) if empty_cols[i] is True]

def expand_universe(strings:list[str]) -> list[list[str]]:
    universe = [ list(string) for string in strings]

    galaxies = find_galaxies(universe)
    width,height = len(universe[0]), len(universe)
    
    empty_rows = find_all_empty_rows(width, galaxies)
    empty_cols = find_all_empty_columns(height, galaxies)

    expanded_universe = []
    for y, row in enumerate(universe):
        new_row = []
        for x, value in enumerate(row):
            new_row.append(value)
            if x in empty_cols:
                new_row.append(value)
        
        expanded_universe.append(new_row)
        if y in empty_rows:
            expanded_universe.append(new_row)

    return expanded_universe


def hexpand_galaxies(galaxies:set[Loc], empty_cols:list[int], expansion_factor:int=1) -> set[Loc]:
    expanded_galaxies:set[Loc] = set()
    for galaxy in galaxies:
        expansions = 0
        for col in empty_cols:
            if col < galaxy.x:
                expansions += 1
        x = galaxy.x + (expansion_factor*expansions)
        expanded_galaxies.add(Loc(x,galaxy.y))
    return expanded_galaxies

def vexpand_galaxies(galaxies:set[Loc], empty_rows:list[int], expansion_factor:int=1) -> set[Loc]:
    expanded_galaxies:set[Loc] = set()
    for galaxy in galaxies:
        expansions = 0
        for row in empty_rows:
            if row < galaxy.y:
                expansions += 1
        y= galaxy.y + (expansion_factor*expansions)
        expanded_galaxies.add(Loc(galaxy.x,y))
    return expanded_galaxies
    

def manhattan_distance(g1:Loc, g2:Loc) -> int:
    return abs(g1.x-g2.x) + abs(g1.y-g2.y)

def print_universe(universe:list[list[str]]):
    print('\nUNIVERSE\n')
    for row in universe:
        print(''.join(row))
    print("\n\n")


def part1(strings: list[str]) -> int:
    result: int = 0
    universe = expand_universe(strings)
    galaxies = find_galaxies(universe)
    for g1,g2 in combinations(galaxies,2):
        result += manhattan_distance(g1,g2)
    return result

def part1_eff(strings:list[str]) -> int:
    result:int = 0
    
    universe = [list(row) for row in strings]
    width, height = len(universe[0]), len(universe)
    
    galaxies = find_galaxies(universe)
    empty_cols = find_all_empty_columns(width, galaxies)
    empty_rows = find_all_empty_rows(height, galaxies)

    galaxies = hexpand_galaxies(galaxies, empty_cols)
    galaxies = vexpand_galaxies(galaxies, empty_rows)

    for g1,g2 in combinations(galaxies,2):
        result += manhattan_distance(g1,g2)
    return result

def part2(strings: list[str], factor:int) -> int:
    result:int = 0
    
    universe = [list(row) for row in strings]
    width, height = len(universe[0]), len(universe)
        
    galaxies = find_galaxies(universe)
    empty_cols = find_all_empty_columns(width, galaxies)
    empty_rows = find_all_empty_rows(height, galaxies)
    
    galaxies = hexpand_galaxies(galaxies, empty_cols, factor)
    galaxies = vexpand_galaxies(galaxies, empty_rows, factor)
    
    for g1,g2 in combinations(galaxies,2):
        result += manhattan_distance(g1,g2)
    return result


class TestAoC01(unittest.TestCase):

    def test_basic_functions(self):
        strings = SAMPLE_INPUT_1.splitlines()
        universe = [ list(string) for string in strings]

        galaxies = find_galaxies(strings)
        self.assertEqual(9, len(galaxies))
        
        width,height = len(universe[0]), len(universe)
        self.assertEqual(10,width)
        self.assertEqual(10,height)

        empty_rows = find_all_empty_rows(width, galaxies)
        self.assertEqual(2, len(empty_rows))
        self.assertIn(3, empty_rows)
        self.assertIn(7, empty_rows)

        empty_cols = find_all_empty_columns(height, galaxies)
        self.assertEqual(3, len(empty_cols))
        self.assertIn(2, empty_cols)
        self.assertIn(5, empty_cols)
        self.assertIn(8, empty_cols)

    def test_expand_universe(self):
        strings = SAMPLE_INPUT_1.splitlines()
        universe = expand_universe(strings)

        galaxies = find_galaxies(universe)
        self.assertEqual(9, len(galaxies))

        width,height = len(universe[0]), len(universe)
        self.assertEqual(13,width)
        self.assertEqual(12,height)

        empty_rows = find_all_empty_rows(height, galaxies)
        self.assertEqual(4, len(empty_rows))
        self.assertIn(3, empty_rows)
        self.assertIn(4, empty_rows)
        self.assertIn(8, empty_rows)
        self.assertIn(9, empty_rows)

        empty_cols = find_all_empty_columns(width, galaxies)
        self.assertEqual(6, len(empty_cols))
        self.assertIn(2, empty_cols)
        self.assertIn(3, empty_cols)
        self.assertIn(6, empty_cols)
        self.assertIn(7, empty_cols)
        self.assertIn(10, empty_cols)
        self.assertIn(11, empty_cols)

    def test_expand_galaxies(self):
        universe = [list(row.strip()) for row in SAMPLE_INPUT_1.splitlines()]
        width,height = len(universe[0]), len(universe)

        galaxies = find_galaxies(universe)
        empty_cols = find_all_empty_columns(height, galaxies)
        empty_rows = find_all_empty_rows(width, galaxies)

        galaxies = hexpand_galaxies(galaxies, empty_cols)
        galaxies = vexpand_galaxies(galaxies, empty_rows)

        self.assertIn(Loc(x=9, y=10),galaxies)
        self.assertIn(Loc(x=4, y=0),galaxies)
        self.assertIn(Loc(x=12, y=7),galaxies)
        self.assertIn(Loc(x=5, y=11),galaxies)
        self.assertIn(Loc(x=0, y=2),galaxies)
        self.assertIn(Loc(x=1, y=6),galaxies)
        self.assertIn(Loc(x=9, y=1),galaxies)
        self.assertIn(Loc(x=8, y=5),galaxies)
        self.assertIn(Loc(x=0, y=11),galaxies)

    #@unittest.skip("temp")
    def test_part1_sample_input(self):
        self.assertEqual(374, part1(SAMPLE_INPUT_1.splitlines()))

    def test_part1_sample_input_effmemory(self):
        self.assertEqual(374, part1_eff(SAMPLE_INPUT_1.splitlines()))

    #@unittest.skip("temp")
    def test_part1_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part1(lines))

    #@unittest.skip("temp")
    def test_part2_sample_input(self):
        self.assertEqual(1030, part2(SAMPLE_INPUT_2.splitlines(),9))
        self.assertEqual(8410, part2(SAMPLE_INPUT_2.splitlines(),99))

    #@unittest.skip("temp")
    def test_part2_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part2(lines,1000000-1))


if __name__ == '__main__':
    unittest.main()
