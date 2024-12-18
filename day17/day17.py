import re

with open('input.txt') as fin:
    program = list(map(int, re.findall(r'\d+', fin.read())[3:]))

def find(program, ans):
    if program == []: return ans

    for b in range(8):
        a = (ans << 3) + b
        b = b ^ 3
        c = a >> b
        b = b ^ c
        b = b ^ 5
        if b % 8 == program[-1]:
            sub = find(program[:-1], a)
            if sub is None: continue
            return sub

print(find(program, 0))
