from collections import deque
from typing import List, Set, NamedTuple


import unittest

SAMPLE_INPUT_1 = """\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"""

SAMPLE_INPUT_2 = SAMPLE_INPUT_1

class ScratchCard(NamedTuple):
    id:int
    winners:Set[int]
    picks:Set[int]

def read_scratchcard(card_data:str) -> ScratchCard:
    id_str, draws_str = card_data.split(':')
    winners_str, picks_str = draws_str.split("|")

    winners = {int(winner_str) for winner_str in winners_str.split()}
    picks = { int(pick_str) for pick_str in picks_str.split()}
    return ScratchCard(int(id_str[5:]), winners, picks)

def get_scratchcard_points(card:ScratchCard) -> int:
    intersection = card.winners & card.picks
    if len(intersection) <= 0:
        return 0
    return 2**(len(intersection)-1)

def get_scratchcard_wins(card:ScratchCard) -> List[int]:
    wins_count =  len(card.winners & card.picks)
    return [card.id + i for i in range(1,wins_count+1)]

def part1(card_strings: List[str]) -> int:
    result: int = 0
    for card_string in card_strings:
        card = read_scratchcard(card_string)
        result += get_scratchcard_points(card)
    return result


def part2(card_strings: List[str]) -> int:
    result: int = 0

    cards = {}
    for card_string in card_strings:
        card = read_scratchcard(card_string)
        cards[card.id] = card

    q = deque(cards.keys())
    
    while q:
        result += 1
        card_id = q.popleft()
        q.extend(get_scratchcard_wins(cards[card_id]))

    return result


class TestAoC01(unittest.TestCase):

    #@unittest.skip("temp")
    def test_part1_sample_input(self):
        self.assertEqual(13, part1(SAMPLE_INPUT_1.splitlines()))

    #@unittest.skip("temp")
    def test_part1_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part1(lines))

    #@unittest.skip("temp")
    def test_part2_sample_input(self):
        self.assertEqual(30, part2(SAMPLE_INPUT_2.splitlines()))

    #@unittest.skip("temp")
    def test_part2_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part2(lines))


if __name__ == '__main__':
    unittest.main()
