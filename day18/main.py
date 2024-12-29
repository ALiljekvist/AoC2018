def pad(area: list[list[str]], pad_with='x') -> list[list[str]]:
    for row in area:
        row.insert(0, pad_with)
        row.append(pad_with)
    area.insert(0, [pad_with for _ in range(len(area[0]))])
    area.append([pad_with for _ in range(len(area[0]))])
    return area

def tick(area: list[list[str]]) -> list[list[str]]:
    new_area = [['x' for _ in range(len(r))] for r in area]
    for r in range(1, len(area)-1):
        for c in range(1, len(area[r])-1):
            # Check adjacent
            count = {'.': 0, '|': 0, '#': 0, 'x': 0}
            for dr in range(-1, 2):
                for dc in range(-1, 2):
                    if dr == 0 and dc == 0:
                        continue
                    count[area[r+dr][c+dc]] += 1
            match area[r][c]:
                case '.':
                    new_area[r][c] = '|' if count['|'] >= 3 else '.'
                case '|':
                    new_area[r][c] = '#' if count['#'] >= 3 else '|'
                case '#':
                    new_area[r][c] = '.' if count['#'] < 1 or count['|'] < 1 else '#'

    return new_area

def print_area(area: list[list[str]]):
    for r in area:
        row = ''
        for c in r:
            row += c
        print(row)

def resource_value(area: list[list[str]]) -> int:
    trees = sum([sum([1 for y in r if y == '|']) for r in area])
    lumber = sum([sum([1 for y in r if y == '#']) for r in area])
    return trees*lumber

def look_for_period(values: list[int], val: int) -> int:
    occs = [x for (x, v) in enumerate(values) if v == val]
    p = occs[-1]-occs[-2]
    if all([values[x-p] == values[x] for x in range(occs[-2], occs[-1]+1)]):
        return p
    return -1

def run():
    with open('input.txt') as f:
        data = [[y for y in x.strip()] for x in f.readlines()]
    area = pad(data)
    values = [resource_value(area)]
    for _ in range(10):
        area = tick(area)
        values.append(resource_value(area))
    print('part1:', resource_value(area))

    ticks = 1000000000
    for _ in range(ticks-10):
        area = tick(area)
        val = resource_value(area)
        values.append(val)
        if values.count(val) > 2:
            p = look_for_period(values, val)
            if p > 0:
                break
    ticks_left = ticks-len(values)
    for _ in range(ticks_left%p+1):
        area = tick(area)
    print('part2:', resource_value(area))

if __name__ == '__main__':
    run()
