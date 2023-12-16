from collections import Counter
from enum import Enum
from functools import cmp_to_key

import unittest


SAMPLE_INPUT_1 = """\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"""

SAMPLE_INPUT_2 = SAMPLE_INPUT_1


class HandRanking(Enum):
    HIGH_CARD = 1
    ONE_PAIR = 2
    TWO_PAIR = 3
    THREE_OF_KIND = 4
    FULL_HOUSE = 5
    FOUR_OF_KIND = 6
    FIVE_OF_KIND = 7


def rank_hand(hand: list[int], useWild: bool) -> HandRanking:
    rank = HandRanking.HIGH_CARD
    counts = Counter(hand)

    if useWild and 11 in counts and len(counts) > 1:
        jokers = counts[11]
        del counts[11]
        most_common = counts.most_common(1)[0]
        counts[most_common[0]] += jokers

    match len(counts):
        case 1:
            rank = HandRanking.FIVE_OF_KIND
        case 2:
            if 2 in counts.values():
                rank = HandRanking.FULL_HOUSE
            else:
                rank = HandRanking.FOUR_OF_KIND
        case 3:
            if 3 in counts.values():
                rank = HandRanking.THREE_OF_KIND
            else:
                rank = HandRanking.TWO_PAIR
        case 4:
            rank = HandRanking.ONE_PAIR

    return rank


def read_hand_and_bid(line: str) -> tuple[list[int], int]:
    hand_data, bid_data = line.split()
    bid = int(bid_data)
    hand = []
    for c in hand_data:
        match c:
            case '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9':
                hand.append(int(c))
            case 'T':
                hand.append(10)
            case 'J':
                hand.append(11)
            case 'Q':
                hand.append(12)
            case 'K':
                hand.append(13)
            case 'A':
                hand.append(14)

    return (hand, bid)


def camel_sort(hand1: list[int], hand2: list[int], useWild: bool) -> int:
    if rank_hand(hand1, useWild) == rank_hand(hand2, useWild):
        return fallback_camel_sort(hand1, hand2, useWild)
    else:
        return rank_hand(hand1, useWild).value - rank_hand(hand2, useWild).value


def fallback_camel_sort(hand1: list[int], hand2: list[int], useWild: bool) -> int:
    for i in range(0, len(hand1)):
        h1 = hand1[i]
        h2 = hand2[i]
        if useWild:
            if h1 == 11:
                h1 = 1
            if h2 == 11:
                h2 = 1
        if h1 != h2:
            return h1 - h2

    return 0


def part1(strings: list[str]) -> int:
    hands_and_bids = [read_hand_and_bid(line) for line in strings]
    cmp: callable[[tuple[int], tuple[int]],
                  int] = lambda hb1, hb2: camel_sort(hb1[0], hb2[0], False)
    hands_and_bids.sort(key=cmp_to_key(cmp))
    return sum([hand_and_bid[1]*(i+1) for i, hand_and_bid in enumerate(hands_and_bids)])


def part2(strings: list[str]) -> int:
    hands_and_bids = [read_hand_and_bid(line) for line in strings]
    cmp: callable[[tuple[int], tuple[int]],
                  int] = lambda hb1, hb2: camel_sort(hb1[0], hb2[0], True)
    hands_and_bids.sort(key=cmp_to_key(cmp))
    # for i, hands_and_bid in enumerate(hands_and_bids):
    # hand = hands_and_bid[0]
    # bid = hands_and_bid[1]
    # rank = rank_hand(hand, True)
    # print( f'{i+1} \t {hand} \t {rank} \t {bid}')
    return sum([hand_and_bid[1]*(i+1) for i, hand_and_bid in enumerate(hands_and_bids)])


class TestAoC01(unittest.TestCase):

    def test_read_handbid_and_ranking(self):
        hb1 = read_hand_and_bid("32T3K 765")
        hb2 = read_hand_and_bid("T55J5 684")
        hb3 = read_hand_and_bid("KK677 28")
        hb4 = read_hand_and_bid("KTJJT 220")
        hb5 = read_hand_and_bid("QQQJA 483")

        self.assertEqual(([3, 2, 10, 3, 13], 765), hb1)
        self.assertEqual(([10, 5, 5, 11, 5], 684), hb2)
        self.assertEqual(([13, 13, 6, 7, 7], 28), hb3)
        self.assertEqual(([13, 10, 11, 11, 10], 220), hb4)
        self.assertEqual(([12, 12, 12, 11, 14], 483), hb5)

        self.assertEqual(HandRanking.ONE_PAIR, rank_hand(hb1[0], False))
        self.assertEqual(HandRanking.THREE_OF_KIND, rank_hand(hb2[0], False))
        self.assertEqual(HandRanking.TWO_PAIR, rank_hand(hb3[0], False))
        self.assertEqual(HandRanking.TWO_PAIR, rank_hand(hb4[0], False))
        self.assertEqual(HandRanking.THREE_OF_KIND, rank_hand(hb5[0], False))

        self.assertEqual(HandRanking.FIVE_OF_KIND,
                         rank_hand([3, 3, 3, 3, 3], False))
        self.assertEqual(HandRanking.FOUR_OF_KIND,
                         rank_hand([3, 3, 3, 3, 5], False))
        self.assertEqual(HandRanking.FULL_HOUSE,
                         rank_hand([5, 4, 4, 4, 5], False))
        self.assertEqual(HandRanking.HIGH_CARD,
                         rank_hand([13, 12, 11, 10, 9], False))

        self.assertEqual(HandRanking.HIGH_CARD,
                         rank_hand([10, 9, 8, 7, 6], True))

        self.assertEqual(HandRanking.ONE_PAIR,
                         rank_hand([11, 9, 8, 7, 6], True))
        self.assertEqual(HandRanking.THREE_OF_KIND,
                         rank_hand([11, 9, 9, 8, 7], True))
        self.assertEqual(HandRanking.FULL_HOUSE,
                         rank_hand([11, 9, 9, 8, 8], True))
        self.assertEqual(HandRanking.FOUR_OF_KIND,
                         rank_hand([11, 9, 9, 9, 8], True))
        self.assertEqual(HandRanking.FIVE_OF_KIND,
                         rank_hand([11, 9, 9, 9, 9], True))

        self.assertEqual(HandRanking.THREE_OF_KIND,
                         rank_hand([13, 11, 11, 10, 9], True))
        self.assertEqual(HandRanking.FOUR_OF_KIND,
                         rank_hand([13, 11, 11, 13, 9], True))
        self.assertEqual(HandRanking.FIVE_OF_KIND,
                         rank_hand([13, 11, 11, 13, 13], True))

        self.assertEqual(HandRanking.FIVE_OF_KIND,
                         rank_hand([13, 11, 11, 11, 13], True))
        self.assertEqual(HandRanking.FOUR_OF_KIND,
                         rank_hand([13, 11, 11, 11, 10], True))

        self.assertEqual(HandRanking.FIVE_OF_KIND,
                         rank_hand([11, 11, 11, 11, 11], True))

    def test_camel_wildcard_fallback_sort(self):
        input = """\
            246Q6 930
            24J97 724
            24J9K 181"""
        hands_and_bids = [read_hand_and_bid(line)
                          for line in input.splitlines()]
        hands_and_bids.sort(key=cmp_to_key(
            lambda hb1, hb2: camel_sort(hb1[0], hb2[0], True)))

        self.assertEqual(hands_and_bids[0][1], 724)
        self.assertEqual(hands_and_bids[1][1], 181)
        self.assertEqual(hands_and_bids[2][1], 930)

    def test_camel_sort(self):
        hands_and_bids = [read_hand_and_bid(line)
                          for line in SAMPLE_INPUT_1.splitlines()]
        self.assertEqual(hands_and_bids[0][1], 765)
        self.assertEqual(hands_and_bids[1][1], 684)
        self.assertEqual(hands_and_bids[2][1], 28)
        self.assertEqual(hands_and_bids[3][1], 220)
        self.assertEqual(hands_and_bids[4][1], 483)

        hands_and_bids.sort(key=cmp_to_key(
            lambda hb1, hb2: camel_sort(hb1[0], hb2[0], False)))
        self.assertEqual(hands_and_bids[0][1], 765)
        self.assertEqual(hands_and_bids[1][1], 220)
        self.assertEqual(hands_and_bids[2][1], 28)
        self.assertEqual(hands_and_bids[3][1], 684)
        self.assertEqual(hands_and_bids[4][1], 483)

    # @unittest.skip("temp")

    def test_part1_sample_input(self):
        self.assertEqual(6440, part1(SAMPLE_INPUT_1.splitlines()))

    # @unittest.skip("temp")
    def test_part1_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part1(lines))

    # @unittest.skip("temp")
    def test_part2_sample_input(self):
        self.assertEqual(5905, part2(SAMPLE_INPUT_2.splitlines()))

    # @unittest.skip("temp")
    def test_part2_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part2(lines))


if __name__ == '__main__':
    unittest.main()
