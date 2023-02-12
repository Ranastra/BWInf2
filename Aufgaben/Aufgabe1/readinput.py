
NUMBER_TESTCASES = 7

testcases = []
for i in range(1,NUMBER_TESTCASES+1):
    s = str(__file__)[:-13] + "/testcases/bsp" + str(i) + ".txt"
    with open(s, encoding="utf-8-sig") as test:
        lines = test.readlines()
        lines = [tuple([float(num) for num in line.split()]) for line in lines]
        testcases.append(lines)
        test.close()

def print_points(testcases, i):
    print("test", i)
    print("X", i)
    for point in testcases[i]:
        print(point[0])
    print("Y", i)
    for point in testcases[i]:
        print(point[1])

if __name__ == "__main__":
    #print(testcases[0][:20]) #debug
    print_points(testcases, 6)