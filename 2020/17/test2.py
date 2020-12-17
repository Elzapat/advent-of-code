import collections
import itertools

dim = 4
active_cubes = [(i, j) + (0,) * (dim-2) for i, line in enumerate(open('17.txt')) for j, c in enumerate(line) if c == '#']
for _ in range(6):
    neighbor_counts = collections.defaultdict(lambda: 0) 
    for cube in active_cubes:
        for offset in itertools.product(range(-1, 2), repeat=dim):
            if not any(offset):
                continue
            neighbor_counts[tuple(a + b for a, b in zip(cube, offset))] += 1
    new_active = []
    active_cubes = set(active_cubes)
    for cube, count in neighbor_counts.items():
        if cube in active_cubes:
            if count in (2, 3):
                new_active.append(cube)
        elif count == 3:
            new_active.append(cube)
    active_cubes = new_active
    print(len(new_active))
