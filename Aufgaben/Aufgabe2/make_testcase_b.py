from random import randint
from sys import argv

def read(path:str) -> list[str]:
    with open(path, "r") as file:
        lines:list[str] = file.readlines()[1:]
    return lines

def remove_random(testcase:list[str], n:int, slice_to_not_pick:str) -> list[str]:
    removed:list[list[int]] = []
    slice_to_not_pick = slice_to_not_pick.replace(",", "")
    retry_count = 0
    for _ in range(n):
        rem = testcase.pop(randint(0, len(testcase)-1))
        while rem == slice_to_not_pick:
           retry_count += 1
           if retry_count > 30:
               print("skipped because not enogh slices")
               return []
           testcase.append(rem)
           rem = testcase.pop(randint(0, len(testcase)-1))
        removed.append(rem)
    print("removed:\n", removed)
    return removed

def output(path:str, testcase:list[str]):
    with open(path, "w") as file:
        file.write(str(len(testcase)) + "\n")
        for slice in testcase:
            file.write(slice)

if __name__ == "__main__":
    if len(argv) >= 2:
        if argv[1] == "all":
            for i in range(1, 8):
                testcase: list[str] = read(f"testcases/bsp{i}.txt")
                slice_to_not_pick: str = read(f"output/bsp{i}.txt")[0]
                removed:list[str] = remove_random(testcase, n=int(argv[2]), slice_to_not_pick=slice_to_not_pick)
                output(path=f"testcasesb/bsp{i}.txt", testcase=testcase)
        else:
            print(f"argv[1]: {argv[1]}")
            testcase: list[str] = read(f"testcases/{argv[1]}")
            slice_to_not_pick: str = read(f"output/{argv[1]}")[0]
            removed:list[str] = remove_random(testcase, n=int(argv[2]), slice_to_not_pick=slice_to_not_pick)
            output(path=f"testcasesb/{argv[1]}", testcase=testcase)
    else:
        for i in range(1, 8):
            testcase: list[str] = read(f"testcases/bsp{i}.txt")
            slice_to_not_pick: str = read(f"output/bsp{i}.txt")[0]
            #print(testcase)
            removed:list[str] = remove_random(testcase, n=1, slice_to_not_pick=slice_to_not_pick)
            #print("removed:\n", removed)
            output(path=f"testcasesb/bsp{i}.txt", testcase=testcase)
            # print(testcase)