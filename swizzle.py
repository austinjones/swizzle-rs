import random
import collections
import inspect

def shift(seq, n=0):
    a = n % len(seq)
    return seq[-a:] + seq[:-a]


def overflow(n):
    if n < -9999:
        return -9999

    if n > 9999:
        return 9999

    return n


def addi(t, a):
    return overflow(t + a)


def subi(t, a):
    return overflow(t - a)


def muli(t, a):
    return overflow(t * a)


def divi(t, a):
    return overflow(t / a)

class Fn:
    def __init__(self, name, impl):
        self.name = name
        self.impl = impl

class Fns:
    def __init__(self, fns):
        def _compose_fn(fns, a):
            point = a
            for fn in fns:
                point = fn.impl(point)

            return point

        self.fns = fns
        self.impl = lambda a: _compose_fn(fns, a)
        self.desc = [f.name for f in fns]

    def append(self, fn):
        return Fns(self.fns + [fn])

    def eval(self, a):
        return self.impl(a)

digits = range(0, 10)
targets = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9]
adds = [Fn("add({})".format(a), lambda t: addi(t, a)) for a in range(0, 10000)]
subs = [Fn("subs({})".format(a), lambda t: subi(t, a)) for a in range(0, 10000)]
muls = [Fn("muls({})".format(a), lambda t: muli(t, a)) for a in range(0, 10000)]
divs = [Fn("divs({})".format(a), lambda t: divi(t, a)) for a in range(0, 10000)]
functions = adds + subs + muls + divs

# targets = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
# diff = [a - b for (a, b) in zip(digits,targets)]

def test(fns):
    for digit in digits:
        if fns.eval(digit) != targets[digit]:
            return

    print("Match found: {}".format(fns))



def display(fns):
    return [fn.name for fn in fns]

class Node:
    def __init__(self, name, fns):
        pass

print("Searching...")
def dfs(fns, depth):
    if depth > 2:
        return

    for addfn in functions:
        new = fns.append(addfn)
        test(new)
        dfs(new, depth+1)


dfs(Fns([]), 0)
#
# print(digits)
# print(targets)
# # print(diff)
#
# for digit in digits:
#     print(targets[digit])
#
# for shiftAmount in range(0, 9):
#     shiftedTarget = shift(targets, shiftAmount)
#
#     print("shift {} => {}".format(shiftAmount, shiftedTarget))
#     for mult in range(0, 999999):
#         match = True
#
#         for digit in digits:
#             if (digit * mult) % 10 != shiftedTarget[digit]:
#                 match = False
#                 break
#
#         if match:
#             print("{} => {}".format(mult, match))
