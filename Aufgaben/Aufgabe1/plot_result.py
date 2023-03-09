import matplotlib.pyplot as plt

def plot_path(i):
    with open("output/test" + str(i) + ".txt", "r") as file:
        lines = [line.strip() for line in file.readlines()][:-1]
        distance = lines[0]
        xs = []
        ys = []
        for point in lines[1:]:
            x, y = point.split(", ")
            xs.append(float(x))
            ys.append(float(y))
            #plt.plot(float(x), float(y))
        plt.plot(xs, ys, marker=".")
        plt.suptitle(str(i)+" "+str(distance))
        plt.show()
        file.close()

def plot_points(i):
    with open("testcases/bsp" + str(i) + ".txt", "r") as file:
        lines = [line.strip() for line in file.readlines()]
        xs = []
        ys = []
        for point in lines[1:]:
            x, y = point.split(" ")
            xs.append(float(x))
            ys.append(float(y))
            #plt.plot(float(x), float(y))
        plt.plot(xs, ys, marker=".", linewidth=0)
        plt.suptitle(str(i))
        plt.show()
        file.close()



if __name__ == "__main__":
    for i in range(1,8):
        plot_points(i)
