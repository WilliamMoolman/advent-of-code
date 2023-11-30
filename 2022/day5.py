from utils import line_input
from parse import parse

class Crane:
    def __init__(self) -> None:
        self.stacks = []
    
    def generate_stacks(self, n):
        self.stacks = [[] for _ in range(n)]
    
    def add_crate(self, s, stack):
        self.stacks[stack].insert(0, s)
    
    def cmd9000(self, start, end, num):
        for _ in range(num):
            x = self.stacks[start-1].pop()
            self.stacks[end-1].append(x)

    def cmd9001(self, start, end, num):
        x = self.stacks[start-1][-num:]
        self.stacks[start-1] = self.stacks[start-1][:-num]
        self.stacks[end-1].extend(x)
    
    def tops(self):
        out = ""
        for s in self.stacks:
            if len(s) == 0:
                out += " "
            else:
                out += s[-1]
        return out

def part1():
    starting = True
    generated = False
    c = Crane()
    for line in line_input(5):
        if not generated:
            n = len(line)//4 + 1
            c.generate_stacks(n)
            generated = True
        
        if line == "":
            starting = False
            continue

        if starting:
            for i in range(n):
                s = line[i*4+1:i*4+2]
                if s == " ": continue
                c.add_crate(s, i)
        else:
            cmd = parse("move {} from {} to {}", line)
            c.cmd9000(int(cmd[1]), int(cmd[2]), int(cmd[0]))

    print(c.tops())
    
def part2():
    starting = True
    generated = False
    c = Crane()
    for line in line_input(5):
        if not generated:
            n = len(line)//4 + 1
            c.generate_stacks(n)
            generated = True
        
        if line == "":
            starting = False
            continue

        if starting:
            for i in range(n):
                s = line[i*4+1:i*4+2]
                if s == " ": continue
                c.add_crate(s, i)
        else:
            cmd = parse("move {} from {} to {}", line)
            c.cmd9001(int(cmd[1]), int(cmd[2]), int(cmd[0]))

    print(c.tops())

part1()
part2()