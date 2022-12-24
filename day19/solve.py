import re
import sys

"""
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.
"""

"""
30 geodes, 3 obsidian robots, 1 ore robot,
actions:
    - buy ore robot
    - buy clay robot
    - buy obsidian robot
    - buy geode robot
    - wait
24 moves
"""


def parse_blueprint(blueprint):
    lines = blueprint.splitlines()[1:]
    costs = {}
    for line in lines:
        type_regex = r"Each (\w+)"
        costs_regex = r"(\d+) (\w+)"
        # get all matches
        type_match = re.search(type_regex, line)
        assert type_match
        matches = re.findall(costs_regex, line)
        kind = type_match.group(1)
        costs[kind] = {k: int(v) for v, k in matches}
    return costs


def parse(input):
    # first, do "each _ costs ____"
    # then do _ x and _ y
    blueprints = input.split("\n\n")
    return [parse_blueprint(b) for b in blueprints]


def part1(input):
    costs = parse(input)
    print(costs)


if __name__ == "__main__":
    # read stdin
    input = sys.stdin.read()
    part1(input)
