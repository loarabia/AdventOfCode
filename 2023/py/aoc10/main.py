import unittest

SAMPLE_INPUT_1 = """\
.....
.S-7.
.|.|.
.L-J.
.....
"""

SAMPLE_INPUT_2 = """\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"""

SAMPLE_INPUT_3 = """\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"""

SAMPLE_INPUT_4 = """\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"""

SAMPLE_INPUT_5 = """\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
"""


def find_start(graph: list[list[str]]) -> tuple[int, int]:
    for y, row in enumerate(graph):
        for x, cell in enumerate(row):
            if cell == 'S':
                return (x, y)
    raise Exception("No Start Node")


def convert_start_symbol(start: tuple[int, int], neighbors: list[tuple[int, int]]) -> str:
    n1 = []
    for neighbor in neighbors:
        if neighbor[0] < start[0]:
            n1.append('N')
        elif neighbor[0] > start[0]:
            n1.append('S')
        elif neighbor[1] < start[1]:
            n1.append('W')
        else:
            n1.append('E')

    result = ''
    n = (n1[0], n1[1])
    match n:
        case ('N', 'S') | ('S', 'N'):
            result = '|'
        case ('W', 'E') | ('E', 'W'):
            result = '-'
        case ('N', 'W') | ('W', 'N'):
            result = 'J'
        case ('N', 'E') | ('E', 'N'):
            result = 'L'
        case ('S', 'W') | ('W', 'S'):
            result = '7'
        case ('S', 'E') | ('E', 'S'):
            result = 'F'
    return result


def find_neighbors(graph: list[list[str]], loc: tuple[int, int]) -> list[tuple[int, int]]:
    size = (len(graph[0]), len(graph))

    top = None if loc[1]-1 < 0 else (loc[0], loc[1]-1)
    bot = None if loc[1]+1 >= size[1] else (loc[0], loc[1]+1)
    lft = None if loc[0]-1 < 0 else (loc[0]-1, loc[1])
    rgt = None if loc[0]+1 >= size[0] else (loc[0]+1, loc[1])

    return [node for node in [top, bot, lft, rgt] if node is not None]


def find_start_next(start: tuple[int, int], graph: list[list[str]]) -> list[tuple[int, int]]:
    candidates = find_neighbors(graph, start)

    candidates = [node for node in candidates if next_steps(
        node, graph) is not None and start in next_steps(node, graph)]

    return candidates


def next_steps(loc: tuple[int, int], graph: list[str]) -> None | tuple[tuple[int, int], tuple[int, int]]:
    result: None | tuple[tuple[int, int], tuple[int, int]] = None
    match graph[loc[1]][loc[0]]:
        case '|':
            result = ((loc[0], loc[1]-1), (loc[0], loc[1]+1))
        case '-':
            result = ((loc[0]-1, loc[1]), (loc[0]+1, loc[1]))
        case 'L':
            result = ((loc[0], loc[1]-1), (loc[0]+1, loc[1]))
        case 'J':
            result = ((loc[0], loc[1]-1), (loc[0]-1, loc[1]))
        case '7':
            result = ((loc[0]-1, loc[1]), (loc[0], loc[1]+1))
        case 'F':
            result = ((loc[0]+1, loc[1]), (loc[0], loc[1]+1))
    return result


def pair_it(chars: str) -> tuple[str, str]:
    for i in range(0, len(chars)-1):
        yield (chars[i], chars[i+1])


def expand_row(row: list[str]) -> list[str]:
    new_row = []
    pipe_expansions = {
        ('-', '-'),
        ('-', '7'),
        ('-', 'J'),
        ('L', '-'),
        ('L', '7'),
        ('L', 'J'),
        ('F', '-'),
        ('F', '7'),
        ('F', 'J'),
        ('S', 'J'),
        ('S', '7'),
        ('S', '-'),
        ('-', 'S'),
        ('F', 'S'),
        ('L', 'S')
    }

    last: None | str = None
    for c1, c2 in pair_it(row):
        new_row.append(c1)
        if (c1, c2) in pipe_expansions:
            new_row.append('-')
        else:
            new_row.append('.')
        last = c2
    new_row.append(last)
    return new_row


def expand_graph(strings: list[str], start_symbol: str | None, start_loc: tuple[int, int] | None) -> list[list[str]]:

    # Make the Graph a mutable structure
    graph: list[list[str]] = [list(row) for row in strings]

    if start_loc and start_symbol:
        # Replace the start node so we don't leak out later
        graph[start_loc[1]][start_loc[0]] = start_symbol

    h_expanded_graph = [expand_row(row) for row in graph]

    expanded_graph = []
    for yi in range(0, len(strings)-1):
        r1 = h_expanded_graph[yi]
        r2 = h_expanded_graph[yi+1]
        expanded_graph.append(r1)
        expanded_graph.append(expand_column(r1, r2))
    expanded_graph.append(h_expanded_graph[-1])
    return expanded_graph


def expand_column(above: list[str], below: list[str]) -> list[str]:
    new_row = []
    pipe_expansions = {
        ('|', '|'),
        ('|', 'L'),
        ('|', 'J'),
        ('F', '|'),
        ('F', 'L'),
        ('F', 'J'),
        ('7', '|'),
        ('7', 'L'),
        ('7', 'J'),
        ('S', '|'),
        ('S', 'J'),
        ('S', 'L'),
        ('|', 'S'),
        ('F', 'S'),
        ('7', 'S')
    }
    for i in range(0, len(above)):
        if (above[i], below[i]) in pipe_expansions:
            new_row.append('|')
        else:
            new_row.append('.')
    return new_row


def find_pipe(start: tuple[int, int], strings: list[list[str]]) -> dict[tuple[int, int], int]:
    step = 0
    values = {start: step}

    path1, path2 = find_start_next(start, strings)
    while True:
        if path1 == path2:
            values[path1] = step+1
            break

        p11, p12 = next_steps(path1, strings)
        values[path1] = step+1
        if p11 not in values:
            path1 = p11
        elif p12 not in values:
            path1 = p12
        else:
            break

        p21, p22 = next_steps(path2, strings)
        values[path2] = step+1
        if p21 not in values:
            path2 = p21
        elif p22 not in values:
            path2 = p22
        else:
            break

        step += 1
    return values


def sweep_graph(graph: list[list[str]], pipe: set[tuple[int, int]]) -> list[list[str]]:
    for y, row in enumerate(graph):
        for x, value in enumerate(row):
            if (x, y) not in pipe:
                graph[y][x] = '.'
    return graph


def flood_area(graph: list[list[str]], start: tuple[int, int]) -> list[list[str]]:
    is_outer = False

    to_visit: set[tuple[int, int]] = {start}
    visited: set[tuple[int, int]] = set()

    while to_visit:
        node = to_visit.pop()

        neighbors = find_neighbors(graph, node)
        if len(neighbors) < 4:
            is_outer = True
        for neighbor in neighbors:
            if neighbor not in visited and graph[neighbor[1]][neighbor[0]] == '.':
                to_visit.add(neighbor)
        visited.add(node)

    for node in visited:
        graph[node[1]][node[0]] = 'O' if is_outer else 'I'
    return graph


def print_graph(strings: list[list[str]], paths: dict[tuple[int, int], int], showcounts=True):
    print()
    for y, row in enumerate(strings):
        s = []
        for x, value in enumerate(row):
            if (x, y) in paths:
                s.append(str(paths[(x, y)] % 10) if showcounts else '#')
            else:
                s.append(value)
        print(''.join(s))
    print()


def part1(strings: list[str]) -> int:
    result: int = 0
    start: tuple[int, int] = find_start(strings)
    pipe_path = find_pipe(start, strings)

    step = max(pipe_path.values())
    return step


def part2(strings: list[str]) -> int:
    graph = [list(row) for row in strings]

    egraph = expand_graph(graph, None, None)
    start = find_start(egraph)
    pipes = find_pipe(start, egraph)

    egraph = sweep_graph(egraph, pipes)

    empty_spaces = set()
    for y, row in enumerate(egraph):
        for x, value in enumerate(row):
            if value == '.':
                empty_spaces.add((x, y))

    for empty_space in empty_spaces:
        if egraph[empty_space[1]][empty_space[0]] == '.':
            egraph = flood_area(egraph, empty_space)

    inners = set()
    for y, row in enumerate(egraph):
        for x, value in enumerate(row):
            if value == 'I' and y % 2 == 0 and x % 2 == 0:
                inners.add((x, y))

    return len(inners)


class TestAoC01(unittest.TestCase):

    def test_find_start(self):
        self.assertTupleEqual((1, 1), find_start(SAMPLE_INPUT_1.splitlines()))
        self.assertTupleEqual((0, 2), find_start(SAMPLE_INPUT_2.splitlines()))

    def test_find_start_next(self):
        self.assertEqual(2, len(find_start_next(
            (1, 1), SAMPLE_INPUT_1.splitlines())))
        self.assertEqual(2, len(find_start_next(
            (0, 2), SAMPLE_INPUT_2.splitlines())))
        self.assertIn((1, 2), find_start_next(
            (0, 2), SAMPLE_INPUT_2.splitlines()))
        self.assertIn((0, 3), find_start_next(
            (0, 2), SAMPLE_INPUT_2.splitlines()))

    # @unittest.skip("temp")
    def test_part1_sample_input(self):
        self.assertEqual(4, part1(SAMPLE_INPUT_1.splitlines()))

    # @unittest.skip("temp")
    def test_part1_sample_input2(self):
        self.assertEqual(8, part1(SAMPLE_INPUT_2.splitlines()))

    # @unittest.skip("temp")
    def test_part1_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print()
            print("Part 1")
            print(part1(lines))
            print("/Part 1")
            print()

    # @unittest.skip("temp")
    def test_part2_sample_input(self):
        self.assertEqual(8, part2(SAMPLE_INPUT_4.splitlines()))

    # @unittest.skip("temp")
    def test_part2_input(self):
        with open('input.txt') as input:
            lines = input.readlines()
            print()
            print("Part 2")
            print(part2(lines))
            print("/Part 2")


if __name__ == '__main__':
    unittest.main()
