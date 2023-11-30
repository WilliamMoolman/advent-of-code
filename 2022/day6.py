from parse import parse
from utils import line_input


def is_marker(s, n=4):
    return len(set(s)) == n


def part1():
    with open("input/day6.txt") as f:
        marker = f.read(4)
        i = 4
        while True:
            if is_marker(marker):
                print(i)
                break
            marker = marker[1:] + f.read(1)
            i += 1


def part2():
    with open("input/day6.txt") as f:
        marker = f.read(14)
        i = 14
        while True:
            if is_marker(marker, 14):
                print(i)
                break
            marker = marker[1:] + f.read(1)
            i += 1


part1()
part2()
