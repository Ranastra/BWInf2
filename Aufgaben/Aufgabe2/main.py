for i in range(1,8):
    with open("testcases/bsp" + str(i) + ".txt", encoding="utf-8-sig") as file:
        lines = file.readlines()
        lines = lines[1:]
        count= 0
        for line in lines:
            line = line.strip().split()
            count +=1
            if int(line[0]) > int(line[1]):
                print("fail")

        print(count)

