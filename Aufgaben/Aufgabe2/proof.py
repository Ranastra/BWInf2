NUMBER_TESTCASES = 7
DISPLAY_MODE = False
from datetime import datetime as time

def proof():
    for i in range(1,NUMBER_TESTCASES+1):
        l = []
        path = f"{__file__[:-8]}output/bsp{i}.txt"
        path2 = f"{__file__[:-8]}testcases/bsp{i}.txt"
        with open(path, encoding="utf-8-sig") as file:
            lines = file.readlines()
            if not lines: continue
            start = lines[0]
            start = start.strip().split(", ")
            dimensions = [1, int(start[0]), int(start[1])]
            if DISPLAY_MODE: print(dimensions, end="")
            lines = lines[1:]
            for line in lines:
                dimensions.sort()
                line = line.strip().split(", ")
                a, b = [int(num) for num in line]
                l.append((a,b))
                if a == dimensions[0] and b == dimensions[1]:
                    dimensions[2] +=1
                    if DISPLAY_MODE: print("\r", dimensions, end="")
                elif a == dimensions[0] and b == dimensions[2]:
                    dimensions[1] +=1
                    if DISPLAY_MODE: print("\r", dimensions, end="")
                elif a == dimensions[1] and b == dimensions[2]:
                    dimensions[0] +=1
                    if DISPLAY_MODE: print("\r", dimensions, end="")
                else:
                    print("fail")
                    print(i, line, dimensions, a, b)
                    break
            if DISPLAY_MODE: print()
            print(i, dimensions)

def proof2(i):
    path = f"{__file__[:-8]}output/bsp{i}.txt"
    path2 = f"{__file__[:-8]}testcases/bsp{i}.txt"
    with open(path, "r") as file:
        lines = file.readlines()
    with open(path2) as file:
        lines2 = file.readlines()[1:]
    lines = [[int(num) for num in line.strip().split(", ")] for line in lines]
    lines2 = [[int(num) for num in line.strip().split(" ")] for line in lines2]
    for l in lines2:
        try: 
            lines.remove(l)
        except ValueError:
            print(l)
    
    


if __name__ == "__main__":
    #start = time.now()
    #proof()
    #print(time.now() -start)
    proof2(7)
