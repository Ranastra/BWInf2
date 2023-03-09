from numpy_arrays import create_2d_array_np, create_3d_array_np, np
from point0 import Point, construct_points_tuple, distance, angle

# Lösung mit Optimierung der Distanz

class Path():
    def __init__(self, points:list[Point], distances:np.ndarray, angles:np.ndarray):
        self.points:    list[int]  = [point[0] for point in points]
        self.distances: np.ndarray = distances
        self.angles:    np.ndarray = angles
        self.unvisited: set[int]   = set(self.points)
        self.path:      list[int]  = []
        self.len:       int        = len(self.points)
        self.is_finish: bool       = False

    def solve0(self):
        def step(self:Path) ->bool:
            # Rekursive Funktion die in einem Schritt 
            # einen neuen Punkt hinzufügt
            for point in self.unvisited:
                if self.angles[self.path[-2]][self.path[-1]][point]:
                    self.path.append(point)
                else:
                    continue


        for first_point in self.points:
            self.path.append(first_point)
            self.unvisited.remove(first_point)
            for second_point in self.points:
                if first_point == second_point: continue
                self.path.append(second_point)
                self.unvisited.remove(second_point)
                if step(self): return True
                self.path.pop()
                self.unvisited.add(second_point)
            self.path.clear()
            self.unvisited.add(first_point)
        return False


class Data():
    def __init__(self, test:list[tuple[float,float]]):
        self.test:      list[tuple[float,float]] = test
        self.points:    list[Point]              = construct_points_tuple(test)
        self.distances: np.ndarray               = create_2d_array_np(len(self.points))
        self.angle:     np.ndarray               = create_3d_array_np(len(self.points))
        # Distanzen und Winkel ausrechnen
        for p1 in self.points:
            for p2 in self.points:
                if p1[0] == p2[0]: continue
                self.distances[p1[0]][p2[0]] = distance(p1, p2)
        for p1 in self.points:
            for p2 in self.points:
                for p3 in self.points:
                    if p1[0] == p2[0] or p1[0] == p3[0] or p2[0] == p3[0]: continue
                    a = self.distances[p2[0]][p1[0]]
                    b = self.distances[p2[0]][p3[0]]
                    c = self.distances[p1[0]][p3[0]]
                    ang = angle(a, c, b)
                    # print(ang) #debug
                    self.angle[p1[0]][p2[0]][p3[0]] = ang >= 90
        
    def find_way(self):
        pass

        


if __name__ == "__main__":
    test0 = testcases[0]
    test1 = [(0.0,0.0), (0.0,4.0), (3.0,0.0)]
    d = Data(test1)
    a = Point((1,2.3,4.6))
    print(type(a))
    # print(test0[:8])
    # print(d.distances)
    # print(d.angle)
    # print(d.points[0][0])
