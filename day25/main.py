def dist(p: list[int], o: list[int]) -> int:
    return sum([abs(p[i]-o[i]) for i in range(len(p))])

def find_num_constellations(fixed_points: list[list[int]]) -> int:
    constellations = 0
    visited = [False for _ in fixed_points]
    for (i, point) in enumerate(fixed_points):
        if visited[i]:
            continue
        next = [(i, point)]
        while len(next) > 0:
            (ci, curr) = next.pop()
            if visited[ci]:
                continue
            visited[ci] = True
            for (j, other) in enumerate(fixed_points):
                if i == j or visited[j]:
                    continue
                if dist(curr, other) <= 3:
                    next.append((j, other))
        constellations += 1
    return constellations

def run():
    with open('input.txt') as f:
        fixed_points = [[int(x) for x in r.split(',')] for r in f.readlines() if r]
    print(find_num_constellations(fixed_points))

if __name__ == '__main__':
    run()