# Point Klasse
# class Point():
#     id:int = 0

#     def __init__(self, x:float, y:float):
#         self.x:float = x
#         self.y:float = y
#         self.ID:int = self.id
#         self.__incr()
    
#     @classmethod
#     def __incr(cls) -> None:
#         cls.id += 1
    
#     def distance(self, other) ->float:
#         return sqrt(pow(self.x-other.x, 2) + pow(self.y-other.y, 2))

#     def __hash__(self) -> int:
#         return self.ID
    
#     def __repr__(self) -> str:
#         return str((self.x,self.y,self.ID))

# def construct_points_class(test) -> list[Point]:
#     points:list[Point] = []
#     for x, y in test:
#         points.append(Point(x, y))
#     points[0].id = 0
#     return points