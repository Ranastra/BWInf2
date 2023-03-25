import matplotlib.pyplot as plt
from sys import argv

def plot_path(path):
    # gefundenen path zeigen
    with open(path, "r") as file:
        lines = [line.strip() for line in file.readlines()]
        distance = lines[0]
        xs = []
        ys = []
        for point in lines[1:]:
            x, y = point.split(", ")
            xs.append(float(x))
            ys.append(float(y))
        plt.axis("equal")
        plt.plot([xs[0]], [ys[0]], marker="x")
        plt.plot([xs[-1]], [[ys[-1]]], marker="o")
        plt.plot(xs, ys, marker=".")
        plt.suptitle(str(distance))
        plt.show()
        file.close()

def plot_points(i):
    # nur points zeigen
    with open("testcases/bsp" + str(i) + ".txt", "r") as file:
        lines = [line.strip() for line in file.readlines()]
        xs = []
        ys = []
        for point in lines[1:]:
            x, y = point.split(" ")
            xs.append(float(x))
            ys.append(float(y))
        plt.axis("equal")
        plt.plot(xs, ys, marker=".", linewidth=0)
        plt.suptitle(str(i))
        plt.show()
        file.close()

def print_distances(i):
    # in rust berechnete Entfernungen ueberpruefen
    with open(f"output/test{i}.txt", "r") as file:
        lines = [[float(n) for n in point.strip().split(", ")] for point in file.readlines()]
    print(lines)
    lines = lines[1:]
    for i in range(len(lines)-1):
        print(((lines[i][0]-lines[i+1][0])**2 + (lines[i][1]-lines[i+1][1])**2)**0.5)
    print(((lines[0][0]-lines[-1][0])**2 + (lines[0][1]-lines[-1][1])**2)**0.5)

def print_distance_from_file(i):
    # in rust berechnete Entfernung anzeigen
    with open(f"output/test{i}.txt", "r") as file:
        print(file.readline().strip())



if __name__ == "__main__":
    if len(argv) == 2:
        name = argv[1][::-1].split("/",maxsplit=1)[0][::-1]
        path = "output/" + name
        plot_path(path)
