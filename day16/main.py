# 0
def addr(a, b, regs):
    return regs[a] + regs[b]

# 1
def addi(a, b, regs):
    return regs[a] + b

# 2
def mulr(a, b, regs):
    return regs[a] * regs[b]

# 3
def muli(a, b, regs):
    return regs[a] * b

# 4
def banr(a, b, regs):
    return regs[a] & regs[b]

# 5
def bani(a, b, regs):
    return regs[a] & b

# 6
def borr(a, b, regs):
    return regs[a] | regs[b]

# 7
def bori(a, b, regs):
    return regs[a] | b

# 8
def setr(a, _, regs):
    return regs[a]

# 9
def seti(a, _, regs):
    return a

# 10
def gtir(a, b, regs):
    return 1 if a > regs[b] else 0

# 11
def gtri(a, b, regs):
    return 1 if regs[a] > b else 0

# 12
def gtrr(a, b, regs):
    return 1 if regs[a] > regs[b] else 0

# 13
def eqir(a, b, regs):
    return 1 if a == regs[b] else 0

# 14
def eqri(a, b, regs):
    return 1 if regs[a] == b else 0

# 15
def eqrr(a, b, regs):
    return 1 if regs[a] == regs[b] else 0

def extract_nums(s: str):
    start = s.find('[')
    return [int(x) for x in s[start+1:-1].split(', ')]

def run():
    with open('input.txt') as f:
        data = [chunk for chunk in f.read().split('\n\n\n') if chunk]
    
    ops = [addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr]
    possibilities = []

    # Part 1
    for chunk in data[0].split('\n\n'):
        stuff = chunk.split('\n')
        inp = extract_nums(stuff[0])
        curr_op = [int(x) for x in stuff[1].split()]
        out = extract_nums(stuff[2])
        matches = set([i for (i, op) in enumerate(ops) if op(curr_op[1], curr_op[2], inp) == out[curr_op[3]]])
        possibilities.append([curr_op[0], matches])
    print('part1:', sum([1 for x in possibilities if len(x[1]) >= 3]))

    # Part 2

    # Find intersection of all sets for each number
    op_nbrs = [set([i for i in range(len(ops))]) for _ in range(len(ops))]
    for nbr, poss in possibilities:
        op_nbrs[nbr] = op_nbrs[nbr].intersection(poss)

    # Confirm single-entry sets and reduce until they are all one-to-one
    done = [-1 for _ in range(len(op_nbrs))]
    while any([len(x) > 0 for x in op_nbrs]):
        for i in range(len(op_nbrs)):
            if len(op_nbrs[i]) == 1:
                found = op_nbrs[i].pop()
                for j in range(len(op_nbrs)):
                    if found not in op_nbrs[j]:
                        continue
                    op_nbrs[j].remove(found)
                done[i] = found

    # Parse program and setup registers
    program = [[int(y) for y in x.split()] for x in data[1].split('\n') if x]
    regs = [0, 0, 0, 0]
    for op in program:
        op_code, a, b, c = op[0], op[1], op[2], op[3]
        regs[c] = ops[done[op_code]](a, b, regs)
        # print(regs)
    print('part2:', regs[0])

if __name__ == '__main__':
    run()
