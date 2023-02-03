from copy import copy

def t(n, li:list[list[int]]):
    if n == len(li[0]): return li
    else:
        neuli = []
        for i in range(1, n+1):
            for sli in li:
                if i not in sli:
                    nsli = copy(sli)
                    nsli.append(i)
                    neuli.append(nsli)
        return t(n, neuli)

# a = t(4,[[]])
# print(a)

b = [1,2,3,4,5]

def wenden(n:int, li:list[int]):
    return li[:n] + li[:n:-1]

def check(li:list[int]):
    past = 0
    for num in li:
        if num < past: return False
        else: past = num
    else: return True

#print(b, wenden(0,b))

def pwue(li:list[int]) -> int:
    l = []
    if check(li): return 0
    for i in range(len(li)):
        a = wenden(i, li)
        if check(a): return 1
        else:
            l.append(a)
    return 1 + min([pwue(a) for a in l])
            

def alle(li:list[list[int]], a=0):
    breakers = []
    for l in li:
        n = pwue(l)
        a = max(a, n)
        if a == n:
            breakers.append((l, n))
    breakers.sort()
    for br in breakers:
        if br[1] == a: print(br, ",")


def num(li:list[list[int]]):
    m = 0
    for l in li: m = max(m, pwue(l))
    print(m)
    return 

#alle(t(4,[[]]))
alle(t(9,[[]]))

