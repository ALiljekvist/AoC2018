def fill(
        pos: tuple[int, int],
        water: dict[tuple[int, int], int],
        clay: set[int]
        ) -> dict[tuple[int, int], int]:
    max_y = max([x[1] for x in clay])
    while pos not in water:
        drop = pos
        visited = set()
        while drop[1] <= max_y:
            down = (drop[0], drop[1]+1)
            left = (drop[0]-1, drop[1])
            right = (drop[0]+1, drop[1])
            # Can move down, move down
            if down not in clay and down not in water:
                # New drop, fill from that spot for efficiency
                if left in visited or right in visited:
                    water = fill(drop, water, clay)
                visited.add(drop)
                drop = down
                continue
            if down in water and water[down] > 0:
                water[drop] = 1
                break
            if left not in clay and left not in visited:
                if left not in water:
                    visited.add(drop)
                    drop = left
                    continue
                if water[left] == 1 and right in visited:
                    water[drop] = 1
                    break
            if right in visited:
                water[drop] = 0
                break
            if right not in clay and right not in water:
                    visited.add(drop)
                    drop = right
                    continue
            if right in water:
                water[drop] = water[right]
                if left in water and water[left] == 1:
                    water[drop] = 1
                    while right in water:
                        water[right] = 1
                        right = (right[0]+1, right[1])

                if water[drop] == 1:
                    while left in water:
                        water[left] = 1
                        left = (left[0]-1, left[1])
                break
            else:
                water[drop] = 0
                break
        if drop[1] > max_y:
            water[drop] = 1
        
    return water


def print_field(min_y:int, max_y:int, min_x:int, max_x: int, clay: set[tuple[int,int]], water: dict[tuple[int,int], int]) -> int:
    for i in range(min_y, max_y+1):
        row = ''
        for j in range(min_x, max_x+1):
            if (j,i) in clay:
                row += '#'
                continue
            if (j,i) in water:
                if water[(j,i)] == 1:
                    row += '|'
                else:
                    row += '~'
                continue
            row += '.'
        print(row)

def run():
    PRINT = False
    with open('input.txt') as f:
        data = [s.strip() for s in f.readlines()]

    clay = set()
    for vein in data:
        stuff = vein.split('=')
        stable = int(stuff[1].split(',')[0])
        from_to = [int(x) for x in stuff[2].split('..')]
        match stuff[0]:
            case 'x':
                for i in range(from_to[0], from_to[1]+1):
                    clay.add((stable, i))
            case 'y':
                for i in range(from_to[0], from_to[1]+1):
                    clay.add((i, stable))
    min_y, max_y = min([x[1] for x in clay]), max([x[1] for x in clay])
    water = fill((500, 0), dict(), clay)
    if PRINT:
        min_x, max_x = min([x[0] for x in clay]), max([x[0] for x in clay])
        print_field(min_y, max_y, min_x, max_x, clay, water)
    else:
        print('part1:', sum([1 for x in water.keys() if x[1] >= min_y and x[1] <= max_y]))
        print('part2:', sum([1 for (_, x) in water.items() if x == 0]))

if __name__ == '__main__':
    run()