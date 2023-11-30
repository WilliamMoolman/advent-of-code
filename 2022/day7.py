from utils import line_input
from parse import parse
from typing import Dict
from collections import defaultdict
import queue

class File:
    def __init__(self, name, size) -> None:
        self.name = name
        self.size = size

class Directory:
    def __init__(self, name, parent=None) -> None:
        self.parent: Directory = parent
        self.dirs: Dict[Directory] = {}
        self.files = set()
        self.size = 0
        self.name = name
    
    def add_file(self, file):
        self.size += file.size
        if self.parent is not None:
            self.parent.add_file(file)
    
    def add_dir(self, directory):
        directory.parent = self
        self.dirs[directory.name] = directory

def parse_input():
    root = Directory('/')
    current_dir = root
    for line in line_input(7):
        tokens = line.split()
        if tokens[0] == '$':
            # Command
            if tokens[1] == "cd":
                if tokens[2] == "/":
                    current_dir = root
                elif tokens[2] == "..":
                    current_dir = current_dir.parent
                else:
                    current_dir = current_dir.dirs[tokens[2]]
            elif tokens[1] == "ls":
                continue
            else:
                continue

        elif tokens[0] == "dir":
            current_dir.add_dir(Directory(tokens[1]))
        else:
            size, name = tokens[0], tokens[1]
            current_dir.add_file(File(name, int(size)))
    return root

def part1():
    root = parse_input()
    dirsum = 0
    q = queue.Queue()
    q.put(root)
    visited = defaultdict(lambda: False)
    while not q.empty():
        x: Directory = q.get()
        if x.size < 100000:
            dirsum += x.size
        visited[x] = True
        for _, y in x.dirs.items():
            if not visited[y]:
                q.put(y)

    print(dirsum)
        

def part2():
    root = parse_input()
    delete_size = root.size - 40000000
    print(delete_size)
    smallest_delete = None
    smallest_delete_idx = None
    q = queue.Queue()
    q.put(root)
    visited = defaultdict(lambda: False)
    while not q.empty():
        x: Directory = q.get()
        if x.size > delete_size:
            if smallest_delete is None or x.size < smallest_delete:
                smallest_delete = x.size
                smallest_delete_idx = x.name
        visited[x] = True
        for _, y in x.dirs.items():
            if not visited[y]:
                q.put(y)
    print(smallest_delete)
    print(smallest_delete_idx)
    
part1()
part2()