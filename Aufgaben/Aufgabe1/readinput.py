
NUMBER_TESTCASES = 7

testcases = []
for i in range(1,NUMBER_TESTCASES+1):
    s = str(__file__)[:-13] + "/testcases/bsp" + str(i) + ".txt"
    with open(s, encoding="utf-8-sig") as test:
        lines = test.readlines()
        lines = [tuple([float(num) for num in line.split()]) for line in lines]
        testcases.append(lines)
        test.close()

if __name__ == "__main__":
    print(testcases[0][:20]) #debug