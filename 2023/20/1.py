from dataclasses import dataclass
from enum import Enum, auto
from pprint import pprint

with open("example1") as f:
    lines = f.read().strip().split("\n")

class Signal(Enum):
    HIGH = auto()
    LOW = auto()

@dataclass
class Node:
    def __init__(self):
        self.children: list[str] = []
        self.waiting_to_send = False
    def __repr__(self):
        return self.__class__.__name__ + repr(self.__dict__)

class FlipFlop(Node):
    def __init__(self):
        super().__init__()
        self.on = False
    def make_signal(self, signal: Signal):
        if signal == Signal.LOW:
            self.on = not self.on
            return Signal.HIGH if self.on else Signal.LOW

class Conjuction(Node):
    def __init__(self):
        super().__init__()
        self.parents: dict[str, bool] = {}

# First pass to build nodes and children
nodes = {}
for line in lines:
    left, right = line.split(" -> ")
    if (type := left[0]) in ("%&"):
        left = left[1:]
        if type == "%":
            node = FlipFlop()
        else:
            node = Conjuction()
    else:
        node = Node()
    nodes[left] = node
    for name in right.split(", "):
        nodes[left].children.append(name)

# Second pass to build parents
for line in lines:
    left, right = line.split(" -> ")
    parent = left.lstrip("%&")
    for name in right.split(", "):
        if isinstance((node := nodes[name]), Conjuction):
            node.parents[parent] = False

pprint(nodes)

