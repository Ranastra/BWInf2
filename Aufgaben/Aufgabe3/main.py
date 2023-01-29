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

a = t(4,[[]])
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
    for i in range(len(li)):
        a = wenden(i, li)
        if check(a): return 1
        else:
            l.append(a)
    return 1 + min([pwue(a) for a in l])
            

def alle(li:list[list[int]]):
    for l in li:
        n = pwue(l)
        if n == 5:
            print(l)


def num(li:list[list[int]]):
    m = 0
    for l in li: m = max(m, pwue(l))
    print(m)
    return 

#alle(t(4,[[]]))
alle(t(8,[[]]))
breaker = [8,5,2,7,4,1,6,3]

"""
[8, 5, 2, 7, 4, 1, 6, 3]
[7, 5, 2, 8, 4, 1, 6, 3]
[8, 5, 1, 7, 4, 2, 6, 3]
[7, 5, 1, 8, 4, 2, 6, 3]
[8, 5, 2, 6, 4, 1, 7, 3]
[6, 5, 2, 8, 4, 1, 7, 3]
[8, 5, 1, 6, 4, 2, 7, 3]
[6, 5, 1, 8, 4, 2, 7, 3]
[7, 5, 2, 6, 4, 1, 8, 3]
[6, 5, 2, 7, 4, 1, 8, 3]
[7, 5, 1, 6, 4, 2, 8, 3]
[6, 5, 1, 7, 4, 2, 8, 3]
[8, 4, 2, 7, 3, 6, 1, 5]
[7, 4, 2, 8, 3, 6, 1, 5]
[8, 4, 2, 6, 3, 7, 1, 5]
[6, 4, 2, 8, 3, 7, 1, 5]
[7, 4, 2, 6, 3, 8, 1, 5]
[6, 4, 2, 7, 3, 8, 1, 5]
[8, 4, 1, 7, 3, 6, 2, 5]
[7, 4, 1, 8, 3, 6, 2, 5]
[8, 4, 1, 6, 3, 7, 2, 5]
[6, 4, 1, 8, 3, 7, 2, 5]
[7, 4, 1, 6, 3, 8, 2, 5]
[6, 4, 1, 7, 3, 8, 2, 5]
[8, 4, 1, 7, 2, 6, 3, 5]
[7, 4, 1, 8, 2, 6, 3, 5]
[8, 4, 1, 6, 2, 7, 3, 5]
[6, 4, 1, 8, 2, 7, 3, 5]
[7, 4, 1, 6, 2, 8, 3, 5]
[6, 4, 1, 7, 2, 8, 3, 5]
"""
