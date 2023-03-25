import matplotlib.pyplot as plt

def plot_path(i):
    with open("output/test" + str(i) + ".txt", "r") as file:
        lines = [line.strip() for line in file.readlines()]
        distance = lines[0]
        xs = []
        ys = []
        for point in lines[1:]:
            x, y = point.split(", ")
            xs.append(float(x))
            ys.append(float(y))
        # distance = 0.0
        # for i in range(1, len(xs)):
            # distance += ((xs[i] - xs[i-1])**2 + (ys[i]-ys[i-1])**2)**0.5
        plt.plot([xs[0]], [ys[0]], marker="x")
        plt.plot([xs[-1]], [[ys[-1]]], marker="o")
        plt.plot(xs, ys, marker=".")
        plt.suptitle(str(distance))
        plt.show()
        #print(distance)
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
        plt.plot(xs, ys, marker=".", linewidth=0)
        plt.suptitle(str(i))
        plt.show()
        file.close()

def print_distances(i):
    with open(f"output/test{i}.txt", "r") as file:
        lines = [[float(n) for n in point.strip().split(", ")] for point in file.readlines()]
    print(lines)
    lines = lines[1:]
    for i in range(len(lines)-1):
        print(((lines[i][0]-lines[i+1][0])**2 + (lines[i][1]-lines[i+1][1])**2)**0.5)
    print(((lines[0][0]-lines[-1][0])**2 + (lines[0][1]-lines[-1][1])**2)**0.5)



if __name__ == "__main__":
    for i in range(1,8):
        plot_path(i)
    #print_distances(2)
    #plot_path(2)
