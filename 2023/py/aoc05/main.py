from collections import deque
# Need to learn the latest on typed python. Apparently a good number of
# types I've been using from typing are deprecated...
import unittest

SAMPLE_INPUT_1 = """\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"""

SAMPLE_INPUT_2 = SAMPLE_INPUT_1

MAP_NAMES = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location"
]


def read_seeds(line: str) -> list[int]:
    seeds = []
    _, *seeds_str = line.split()
    for seed_str in seeds_str:
        seeds.append(int(seed_str))
    return seeds


def read_seeds_from_ranges(line: str) -> list[int]:
    seeds = []
    _, *seeds_str = line.split()
    for i in range(0, len(seeds_str), 2):
        seed_start, cnt = int(seeds_str[i]), int(seeds_str[i+1])
        seeds.extend(range(seed_start, seed_start+cnt))
    return seeds


def gen_seeds(line: str):
    _, *seeds_str = line.split()
    for i in range(0, len(seeds_str), 2):
        seed_start, cnt = int(seeds_str[i]), int(seeds_str[i+1])
        for seed in range(seed_start, seed_start+cnt):
            yield seed


def read_seeds_as_ranges(line: str) -> list[range]:
    seed_ranges = []
    _, *seeds_str = line.split()
    for i in range(0, len(seeds_str), 2):
        seed_start, cnt = int(seeds_str[i]), int(seeds_str[i+1])
        seed_ranges.append(range(seed_start, seed_start + cnt))
    return seed_ranges


def read_map(strings: list[str], start: int) -> dict[int, tuple[int, int]]:
    map = {}

    for i in range(start, len(strings)):
        line = strings[i].strip()
        if not line:
            break

        dst, src, rng = [int(val) for val in line.split()]
        map[src] = (dst, rng)
    return map


def read_map_as_range(strings: list[str], start: int):
    map = {}

    for i in range(start, len(strings)):
        line = strings[i].strip()
        if not line:
            break

        dst, src, rng = [int(val) for val in line.split()]
        map[range(src, src+rng)] = range(dst, dst+rng)
    return map


def read_map_locs(strings: list[str], map_names: str) -> dict[str, int]:
    map_locs = {}
    for i, line in enumerate(strings):
        for map_name in map_names:
            if line.startswith(map_name):
                map_locs[map_name] = i
    return map_locs


def lookup(map: dict[int, tuple[int, int]], value: int) -> int:
    translation = value
    for src, meta in map.items():
        dst, len = meta
        if value >= src and value <= (src+len):
            return dst + (value-src)
    return translation


def map_ranges(ranges: list[range], map: dict[range, range]) -> list[range]:
    new_ranges = []
    # There are two ways I can see to do this.
    # I can sort the source ranges in the map and (assuming they don't overlap) build any missing ranges
    # that are 1:1. Then My logic is very uniform with less casing. It takes a little more memory though
    # but not much.
    # OR
    # I could iterate and handle the gaps as special cases which adds more logic but uses less memory.

    # Sort and Fill in map gaps with new 1:1 ranges.
    sorted_sources = list(map.keys())
    sorted_sources.sort(key=lambda src: src.start)

    # Pull off some metadata for now
    src_min = sorted_sources[0].start
    src_max = sorted_sources[len(sorted_sources)-1].stop

    # Add the 1:1 mappings to fill any gaps
    for i, _src in enumerate(sorted_sources):
        # step back by 2 since you're doing a sliding window iteration
        if i >= len(sorted_sources)-1:
            break

        # Is there a gap?
        if sorted_sources[i].stop < sorted_sources[i+1].start:
            new_rng_pair = range(
                sorted_sources[i].stop, sorted_sources[i+1].start)
            map[new_rng_pair] = new_rng_pair

    # You just updated everything so grab it again
    sorted_sources = list(map.keys())
    sorted_sources.sort(key=lambda src: src.start)
    sorted_ranges = ranges
    sorted_ranges.sort(key=lambda rng: rng.start)

    rng_queue = deque(sorted_ranges)
    src_queue = deque(sorted_sources)

    # now is the fun part. Build the new ranges from the input ranges
    # You can either be completely outside the left side of the ranges, in them, on outside.
    # Get to the point where the ranges overlap.

    new_rng_start = None
    new_rng_end = None

    while rng_queue:
        rng = rng_queue.popleft()

        # You're completely off the edge of the map for these first 2 matey
        if rng.stop < src_min or rng.start >= src_max:
            new_ranges.append(rng)
            continue

        if rng.start < src_min and rng.stop > src_min:
            # You're off the left edge of the map
            new_rng_start = rng.start
            new_rng_end = src_min
            new_ranges.append(range(new_rng_start, new_rng_end))
            rng_queue.appendleft(range(src_min+1, rng.stop))
            continue

        elif rng.start < src_max and rng.stop > src_max:
            # You're hanging off the right side of the map.
            # Split that range into two parts, the part that may be in the range and the rest.
            rng_queue.appendleft(range(src_max, rng.stop))
            rng_queue.appendleft(range(rng.start, src_max))

            # And let the machine handle it.
            continue

        # Arrrrr! Its time to walk the map ye dirty scallywag!
        while src_queue:
            source = src_queue.popleft()
            match (rng.start in source, rng.stop-1 in source):
                case (True, True):
                    # Happy path the range you're mapping is a subset
                    #   S...E       is the range
                    # |---------|   is the source from the map
                    #
                    # |--S...E--|   flattened on top of each other
                    offset = rng.start - source.start
                    new_rng_start = map[source].start + offset
                    offset = rng.stop - source.start
                    new_rng_end = map[source].start + offset
                    new_ranges.append(range(new_rng_start, new_rng_end))
                    src_queue.appendleft(source)
                    break

                case (True, False):
                    # We started in the range but end somewhere else
                    # |--S..|..E
                    # Since the mapping could change, we'll generate a range now
                    offset = rng.start - source.start
                    new_rng_start = map[source].start + offset

                    new_rng_end = map[source].stop

                    new_ranges.append(range(new_rng_start, new_rng_end))
                    rng_queue.appendleft(range(source.stop, rng.stop))
                    # Because we've fully consumed this source:dest map we can let it go
                    break

                case (False, True):
                    # GIVEN HOW I'VE WRITTEN THIS, THIS MIGHT NOT BE POSSIBLE
                    # You started outside this range but finish inside it. Kind of a happy path also.
                    # S..|..E--|
                    new_rng_start = map[source].start

                    offset = rng.stop - source.start
                    new_rng_end = map[source].start + offset

                    new_ranges.append(range(new_rng_start, new_rng_end))
                    # TODO maybe put this range back?
                    break

                case (False, False):
                    if source.stop < rng.start:
                        # The range I want to map has already passed by this mapping segment, skip it
                        #
                        #  |----| S...E
                        #
                        continue
                        # In theory, these other cases shouldn't happen.
                        # I could have start and end both out on left side  S...E |----|
                        # I could have start and end both outisde          S..|....|...E
                    break

    return new_ranges


def part1(strings: list[str]) -> int:
    result: int = 0

    seeds = read_seeds(strings[0])
    map_locs = read_map_locs(strings, MAP_NAMES)
    maps = []

    for _name, loc in map_locs.items():
        maps.append(read_map(strings, loc+1))

    for map in maps:
        for i, seed in enumerate(seeds):
            seeds[i] = lookup(map, seed)

    return min(seeds)


def part2(strings: list[str]) -> int:
    result: int = None

    map_locs = read_map_locs(strings, MAP_NAMES)

    maps = []
    for _name, loc in map_locs.items():
        maps.append(read_map_as_range(strings, loc+1))

    seed_ranges = read_seeds_as_ranges(strings[0])

    updated_ranges = seed_ranges
    for map in maps:
        updated_ranges = map_ranges(updated_ranges, map)

    updated_ranges.sort(key=lambda rng: rng.start)
    return updated_ranges[0].start


def part2_brute(strings: list[str]) -> int:
    result: int = None

    map_locs = read_map_locs(strings, MAP_NAMES)

    maps = []
    for _name, loc in map_locs.items():
        maps.append(read_map(strings, loc+1))

    for seed in gen_seeds(strings[0]):
        seed_pos = seed
        for map in maps:
            seed_pos = lookup(map, seed_pos)

        if result is None:
            result = seed_pos
        else:
            result = min(result, seed_pos)
    return result


class TestAoC01(unittest.TestCase):

    def test_read_map_locs(self):
        locs = read_map_locs(SAMPLE_INPUT_1.splitlines(), MAP_NAMES)
        self.assertEqual(7, len(locs))
        self.assertEqual(11, locs["fertilizer-to-water"])

    def test_read_seeds(self):
        seeds = read_seeds("seeds: 79 14 55 13")
        self.assertEqual(4, len(seeds))
        self.assertEqual(79, seeds[0])
        self.assertEqual(13, seeds[3])

    def test_read_seeds_ranges(self):
        seeds = read_seeds_from_ranges("seeds: 79 14 55 13")
        self.assertEqual(27, len(seeds))
        self.assertEqual(79, seeds[0])
        self.assertEqual(92, seeds[13])
        self.assertEqual(55, seeds[14])
        self.assertEqual(67, seeds[len(seeds)-1])

    def test_read_seeds_as_ranges(self):
        seed_ranges = read_seeds_as_ranges("seeds: 79 14 55 13")
        self.assertEqual(2, len(seed_ranges))
        self.assertEqual(27, len(seed_ranges[0]) + len(seed_ranges[1]))
        self.assertEqual(79, seed_ranges[0][0])
        self.assertEqual(92, seed_ranges[0][len(seed_ranges[0])-1])
        self.assertEqual(55, seed_ranges[1][0])
        self.assertEqual(67, seed_ranges[1][len(seed_ranges[1])-1])

    def test_read_map(self):
        map = read_map(SAMPLE_INPUT_1.splitlines(), 3)
        self.assertEqual((50, 2), map[98])

    def test_map_ranges_seed_to_soil(self):
        seed_ranges = read_seeds_as_ranges("seeds: 79 14 55 13")
        map = read_map_as_range(SAMPLE_INPUT_1.splitlines(), 3)
        new_ranges = map_ranges(seed_ranges, map)
        self.assertEqual(len(new_ranges), 2)
        self.assertTrue(range(57, 70) in new_ranges)
        self.assertTrue(range(81, 95) in new_ranges)

    def test_map_ranges_soil_to_fert(self):
        seed_ranges = read_seeds_as_ranges("seeds: 57 13 81 14")
        map = read_map_as_range(SAMPLE_INPUT_1.splitlines(), 7)
        new_ranges = map_ranges(seed_ranges, map)
        self.assertEqual(len(new_ranges), 2)
        self.assertTrue(range(57, 70) in new_ranges)
        self.assertTrue(range(81, 95) in new_ranges)

    def test_map_ranges_fert_to_water(self):
        seed_ranges = read_seeds_as_ranges("seeds: 57 13 81 14")
        map = read_map_as_range(SAMPLE_INPUT_1.splitlines(), 12)
        new_ranges = map_ranges(seed_ranges, map)
        self.assertEqual(len(new_ranges), 3)
        self.assertTrue(range(61, 70) in new_ranges)
        self.assertTrue(range(81, 95) in new_ranges)
        self.assertTrue(range(53, 57) in new_ranges)

    def test_map_ranges_wat_to_light(self):
        seed_ranges = read_seeds_as_ranges("seeds: 53 4 61 9 81 12 ")
        map = read_map_as_range(SAMPLE_INPUT_1.splitlines(), 18)
        new_ranges = map_ranges(seed_ranges, map)
        self.assertEqual(len(new_ranges), 3)
        self.assertTrue(range(46, 50) in new_ranges)
        self.assertTrue(range(54, 63) in new_ranges)
        self.assertTrue(range(74, 86) in new_ranges)

    def test_map_ranges_light_to_temp(self):
        seed_ranges = [range(46, 50), range(54, 63), range(74, 86)]
        map = read_map_as_range(SAMPLE_INPUT_1.splitlines(), 22)
        new_ranges = map_ranges(seed_ranges, map)
        self.assertEqual(len(new_ranges), 4)
        self.assertTrue(range(45, 54) in new_ranges)
        self.assertTrue(range(78, 81) in new_ranges)
        self.assertTrue(range(82, 86) in new_ranges)
        self.assertTrue(range(90, 99) in new_ranges)

    # @unittest.skip("temp")
    def test_part1_sample_input(self):
        self.assertEqual(35, part1(SAMPLE_INPUT_1.splitlines()))

    # @unittest.skip("temp")
    def test_part1_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print(part1(lines))

    # @unittest.skip("temp")
    def test_part2_sample_input_brute(self):
        self.assertEqual(46, part2_brute(SAMPLE_INPUT_2.splitlines()))

    def test_part2_sample_range_mapping(self):
        self.assertEqual(46, part2(SAMPLE_INPUT_2.splitlines()))

    # @unittest.skip("temp")
    def test_part2_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print("PART 2")
            print(part2(lines))
            print("/PART 2")


if __name__ == '__main__':
    unittest.main()
