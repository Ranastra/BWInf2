from numpy_arrays import create_2d_array_np, create_3d_array_np, np
from point0 import Point, construct_points_tuple, distance, angle
from copy import copy
from time import sleep

# Lösung ohne Optimierung der Distanz 

d_mode = False #debug
display_mode = True #debug
display_delay = 0.4

class Path():
    def __init__(self, points:list[Point], distances:np.ndarray, angles:np.ndarray):
        self.points:    list[int]  = [point[0] for point in points]
        self.distances: np.ndarray = distances
        self.angles:    np.ndarray = angles
        self.unvisited: set[int]   = set(self.points)
        self.path:      list[int]  = []

    def solve0(self):
        def step(self:Path) -> bool:
            # Rekursive Funktion die in einem Schritt 
            # einen neuen Punkt hinzufügt
            # Wenn ein passender Weg gefunden wure
            if len(self.unvisited) == 0: return True 
            for point in copy(self.unvisited):
                # Backtracking
                if self.angles[self.path[-2]][self.path[-1]][point]:
                    # Backtracking Schritt
                    if d_mode: print()
                    self.path.append(point)
                    self.unvisited.remove(point)
                    if display_mode: self.display() #debug
                    if step(self):
                        return True
                    else:
                        # Schritt wird wieder rückgängig gemacht
                        if display_mode: self.display() #debug
                        self.unvisited.add(point)
                        self.path.pop()
            else: return False

        # die ersten 2 Punkte werden ausgewählt
        # äußere Schleife 1. Punkt, innere Schleife 2. Punkt
        for first_point in self.points:
            self.path.append(first_point)
            self.unvisited.remove(first_point)
            for second_point in self.points:
                if first_point == second_point: continue
                self.path.append(second_point)
                self.unvisited.remove(second_point)
                if d_mode: print("erste zwei Knoten",self.path) #debug
                if display_mode: self.display() #debug
                if step(self):
                    if display_mode: self.display() #debug
                    if d_mode: print("geht", self.path) #debug
                    return True
                else:
                    if display_mode: self.display() #debug
                    if d_mode: print("geht nicht", self.path) #debug
                    self.path.pop()
                    self.unvisited.add(second_point)
            self.path.clear()
            self.unvisited.add(first_point)
        return False
    
    def display(self, delay=display_delay) -> None:
        to_str = lambda id: (3-len(str(id)))*'0' + str(id)
        s =  ",".join([to_str(id) for id in self.path])
        left_off = len(self.path) - len(self.unvisited)
        s += ",".join(["000" for _ in range(left_off)])
        print("\r", s, end="")
        sleep(delay)
        

class Data():
    def __init__(self, test:list[tuple[float,float]]):
        self.points:    list[Point] = construct_points_tuple(test)
        self.distances: np.ndarray  = create_2d_array_np(len(self.points))
        self.angle:     np.ndarray  = create_3d_array_np(len(self.points))
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
        self.path:Path = Path(self.points, self.distances, self.angle)
        
    def find_way(self) -> str:
        if self.path.solve0():
            return self.__rejoin()
        else:
            return "Es existiert keine passende Flugstrecke"

    def __rejoin(self) -> str:
        fm1 = lambda x:"{:.6f}".format(x)
        fm2 = lambda t: str((fm1(t[1]), fm1(t[2])))
        points = [self.points[id] for id in self.path.path]
        koordinates = [fm2(point) for point in points]
        return ", ".join(koordinates)