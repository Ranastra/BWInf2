NUMBER_TESTCASES = 7
DISPLAY_MODE = False
from datetime import datetime as time

def proof():
    for i in range(1,NUMBER_TESTCASES+1):
        l = []
        with open(__file__[:-8] + "output/test" + str(i) + ".txt", encoding="utf-8-sig") as file:
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


if __name__ == "__main__":
    start = time.now()
    proof()
    print(time.now() -start)
