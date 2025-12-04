import itertools
with open("/Users/michcioperz/Downloads/3.input") as f: data = [[int(x) for x in line.strip()] for line in f]
print(sum(max(x*10+y for x, y in itertools.combinations(line, 2)) for line in data))