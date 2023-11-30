from utils import line_input
from dataclasses import dataclass
from parse import parse


@dataclass
class Schedule:
    start: int = 0
    end: int = 0

    def is_enclosed_by(self, obj):
        return self.start >= obj.start and self.end <= obj.end

    def overlaps_with(self, obj):
        return obj.start <= self.start <= obj.end

def part1():
    num_overlap = 0
    for line in line_input(4, False):
        r = parse("{x[start]}-{x[end]},{y[start]}-{y[end]}", line)
        xs = Schedule(int(r['x']['start']), int(r['x']['end']))
        ys = Schedule(int(r['y']['start']), int(r['y']['end']))
        if xs.is_enclosed_by(ys) or ys.is_enclosed_by(xs):
            # print(xs, ys)
            num_overlap += 1
    print(num_overlap)

def part2():
    num_overlap = 0
    for line in line_input(4, False):
        r = parse("{x[start]}-{x[end]},{y[start]}-{y[end]}", line)
        xs = Schedule(int(r['x']['start']), int(r['x']['end']))
        ys = Schedule(int(r['y']['start']), int(r['y']['end']))
        if xs.overlaps_with(ys) or ys.overlaps_with(xs):
            # print(xs, ys)
            num_overlap += 1
    print(num_overlap)
part1()
part2()
