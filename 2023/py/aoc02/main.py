from typing import List, NamedTuple

import unittest

SAMPLE_INPUT_1 = """\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"""

SAMPLE_INPUT_2 = SAMPLE_INPUT_1

class Draw(NamedTuple):
    id: int
    r: int
    g: int
    b: int


def parse_line(string:str) -> List[Draw]:
    draws = []
    id_str, draws_str = string.split(':')
    id = int(id_str[4:])
    for draw_str in draws_str.split(';'):
        r = 0
        g = 0
        b = 0
        for color_str in draw_str.split(','):
            num_str, color_name = color_str.split()
            match color_name:
                case 'red':
                    r = int(num_str)
                case 'green':
                    g = int(num_str)
                case 'blue':
                    b = int(num_str)
        draw = Draw(id,r,g,b)
        draws.append(draw)
    return draws

def is_possible(max_draw:Draw, sample_draws:List[Draw]) -> bool:
    for sample_draw in sample_draws:
        if sample_draw.r > max_draw.r or sample_draw.g > max_draw.g or sample_draw.b > max_draw.b:
            return False
    return True

def part1(strings: List[str]) -> int:
    max_draw = Draw(0,12,13,14)
    all_games = [parse_line(string) for string in strings]
    possible_games = [ game for game in all_games if is_possible(max_draw, game) ]
    possible_ids = [ game[0].id for game in possible_games]
    return sum(possible_ids)

def get_power(sample_draws:List[Draw]) -> int:
    max_r = 0
    max_g = 0
    max_b = 0
    for sample_draw in sample_draws:
        max_r = max(max_r, sample_draw.r)
        max_g = max(max_g, sample_draw.g)
        max_b = max(max_b, sample_draw.b)

    return max_r * max_g * max_b

def part2(strings: List[str]) -> int:
    all_games = [parse_line(string) for string in strings]
    game_powers = [get_power(game) for game in all_games]
    return sum(game_powers)


class TestAoC01(unittest.TestCase):

    def test_part1_sample_input(self):
        self.assertEqual(8, part1(SAMPLE_INPUT_1.splitlines()))

    def test_part1_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part1(lines))

    def test_part2_sample_input(self):
        self.assertEqual(2286, part2(SAMPLE_INPUT_2.splitlines()))

    def test_part2_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part2(lines))


if __name__ == '__main__':
    unittest.main()
