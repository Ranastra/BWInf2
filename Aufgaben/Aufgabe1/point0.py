from math import acos, degrees, sqrt

# Point Objekte und deren Funktionen

Point = tuple[int, float, float]  # Type Alias

id:int = 0

def point(x:float, y:float) -> Point:
    # Point Konstruktor: point = (id, x, y)
    global id
    p:Point = (id, x, y)
    id += 1
    return p

def construct_points_tuple(test:list[tuple[float, float]]) -> list[Point]:
    # wandelt Input in eine Liste von Point objekten um.
    points:list[Point] = []
    global id
    id = 0
    for x, y in test:
        points.append(point(x,y))
    return points

def distance(p1:Point, p2:Point):
    # Berechnet die Entfernung zwischen 2 Punkten mithilfe des Satz des Pythagoras.
    return sqrt(pow(p1[1]-p2[1], 2) + pow(p1[2]-p2[2], 2))

def angle(a:float, c:float, b:float) -> float:
    # Berechnet den Winkel zwischen den beiden Seiten a und b
    # mithilfe des Cosinussatzes
    deg:float = acos((pow(a,2) + pow(b,2) - pow(c,2))/(2*a*b))
    return round(degrees(deg), ndigits=1)

