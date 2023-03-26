import matplotlib.pyplot as plt
from sys import argv

def plot_path(path):
    # Show the found path
    with open(path, "r") as file:
        # Read and process the contents of the file
        lines = [line.strip() for line in file.readlines()]
        distance = lines[0] # Extract the distance value from the first line
        xs = []
        ys = []
        # Extract the x and y values for each point in the path
        for point in lines[1:]:
            x, y = point.split(", ")
            xs.append(float(x))
            ys.append(float(y))
        # Set the axis limits to be equal to create a square coordinate plane
        plt.axis("equal")
        # Plot the starting and ending points with markers
        plt.plot([xs[0]], [ys[0]], marker="x")
        plt.plot([xs[-1]], [[ys[-1]]], marker="o")
        # Plot the path points
        plt.plot(xs, ys, marker=".")
        # Set the title of the plot to be the total distance of the path
        plt.suptitle(str(distance))
        # Show the plot
        plt.show()

def plot_points(path):
    # open file and read lines, remove any trailing whitespace
    with open(path, "r") as file:
        lines = [line.strip() for line in file.readlines()]
        xs = []
        ys = []
        # extract x and y coordinates from each line
        for point in lines[1:]:
            x, y = point.split(" ")
            xs.append(float(x))
            ys.append(float(y))
        # set the axes to be equal and plot the points
        plt.axis("equal")
        plt.plot(xs, ys, marker=".", linewidth=0)
        # set the title to the file name and display the plot
        plt.suptitle(path)
        plt.show()

def print_distances(path):
    # open file and read lines, remove any trailing whitespace
    with open(path, "r") as file:
        lines = [[float(n) for n in point.strip().split(", ")] for point in file.readlines()]
    # print the coordinates of the points in the file
    print(lines)
    # remove the first line (total distance), 
    # calculate the distance between each pair of adjacent points
    lines = lines[1:]
    for i in range(len(lines)-1):
        print(((lines[i][0]-lines[i+1][0])**2 + (lines[i][1]-lines[i+1][1])**2)**0.5)
    print(((lines[0][0]-lines[-1][0])**2 + (lines[0][1]-lines[-1][1])**2)**0.5)

def print_distance_from_file(path):
    # open file and read the first line (total distance), 
    # remove any trailing whitespace, and print the distance
    with open(path, "r") as file:
        print(file.readline().strip())



if __name__ == "__main__":
    # If there is exactly one command line argument, plot the solution
    if len(argv) == 2 and argv[1] != "all":
         # Extract the filename from the path and generate the output file path
        name = argv[1][::-1].split("/",maxsplit=1)[0][::-1]
        path = "output/" + name
        # Plot the solution using the plot_path function
        plot_path(path)
    # If there are no command line arguments, plot all test cases
    else:
        # Plot the solutions for all 7 test cases
        for i in range(1,8):
            plot_path(f"output/bsp{i}.txt")