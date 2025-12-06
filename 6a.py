import functools, operator

with open("/Users/michcioperz/Downloads/6.input") as f: x = list(f)
y = [line.split() for line in x]
z = [[l[i] for l in y] for i in range(len(y[0]))]
print(sum(functools.reduce(f, map(lambda i: int(i), l[:-1])) for l in z for f in [{'+':operator.add,'*':operator.mul}[l[-1]]]))