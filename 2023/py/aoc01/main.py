from typing import List, Optional

import unittest

SAMPLE_INPUT_1 = """\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"""

SAMPLE_INPUT_2 = """\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""


def remove_alphabet(str: str) -> str:
    return [c for c in str if c >= '0' and c <= '9']

def part1(strings: List[str]) -> int:
    clean = [remove_alphabet(string) for string in strings]
    ints = [int(string[0]+string[-1]) for string in clean]
    return sum(ints)

def part2(strings: List[str]) -> int:
    ints = [int(first(string)+last(string)) for string in strings]
    return sum(ints)


def first(string: str) -> str:
    for i, x in enumerate(string):
        match x:
            case '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9':
                return x
            case 'z':
                if next_tok(string, i, 'ero'):
                    return '0'
            case 'o':
                if next_tok(string, i, 'ne'):
                    return '1'
            case 't':
                if next_tok(string, i, 'wo'):
                    return '2'
                elif next_tok(string, i, 'hree'):
                    return '3'
            case 'f':
                if next_tok(string, i, 'our'):
                    return '4'
                elif next_tok(string, i, 'ive'):
                    return '5'
            case 's':
                if next_tok(string, i, 'ix'):
                    return '6'
                elif next_tok(string, i, 'even'):
                    return '7'
            case 'e':
                if next_tok(string, i, 'ight'):
                    return '8'
            case 'n':
                if next_tok(string, i, 'ine'):
                    return '9'


def next_tok(string: str, start: int, expected: str) -> bool:
    return string[start+1: start + len(expected) + 1] == expected


def next_tok_rev(string: str, start: int, expected: str) -> bool:
    return string[start-len(expected):start] == expected

def last(string: str) -> str:
    for i in range(len(string)-1, -1, -1):
        x = string[i]
        match x:
            case '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9':
                return x
            case 'o':
                if next_tok_rev(string, i, 'zer'):
                    return '0'
                elif next_tok_rev(string, i, 'tw'):
                    return '2'
            case 'e':
                if next_tok_rev(string, i, 'on'):
                    return '1'
                elif next_tok_rev(string, i, 'thre'):
                    return '3'
                elif next_tok_rev(string, i, 'fiv'):
                    return '5'
                elif next_tok_rev(string, i, 'nin'):
                    return '9'
            case 'r':
                if next_tok_rev(string, i, 'fou'):
                    return '4'
            case 'x':
                if next_tok_rev(string, i, 'si'):
                    return '6'
            case 'n':
                if next_tok_rev(string, i, 'seve'):
                    return '7'
            case 't':
                if next_tok_rev(string, i, 'eigh'):
                    return '8'


class TestAoC01(unittest.TestCase):

    def test_part1_sample_input(self):
        self.assertEqual(142, part1(SAMPLE_INPUT_1.splitlines()))

    def test_part1_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part1(lines))

    def test_part2_sample_input(self):
        self.assertEqual(281, part2(SAMPLE_INPUT_2.splitlines()))

    def test_part2_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part2(lines))


if __name__ == '__main__':
    unittest.main()
