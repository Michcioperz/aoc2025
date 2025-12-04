import re
# this is hard to reconstruct from python REPL history where i was heavily using `_`
# so i'll just reimagine how it would be more properly
with open("/Users/michcioperz/Downloads/2.input") as f: data = f.read()
print(sum(i
    for r in data.strip().split(',')
    for x, y in [[int(a) for a in r.split('-')]]
    for i in range(x, y+1)
    for s in [str(i)]
    for l in [len(s)]
    if l % 2 == 0
    if s[:l//2] == s[l//2:]
))

# and for part B i realized there is a beautiful trick
# after i remembered the russ cox rant about exponential regexp implementations
# https://swtch.com/~rsc/regexp/regexp1.html
# (i usually forget that backrefs exist because i have to work in golang
# which doesn't have them for this specific reason i guess??)
print(sum(i
    for e in [re.compile(r'^(.+)\1+$')]
    for r in data.strip().split(',')
    for x, y in [[int(a) for a in r.split('-')]]
    for i in range(x, y+1)
    if e.match(str(i))
))
# after which i realized removing the second + from the regexp gives you a part A solution

# (all of this takes about 2 seconds to run on a M2 Mac Air btw)