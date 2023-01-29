# import der Tests
from readinput import testcases

# import Data class
from path0 import Data

# point objekte für tests #debug
from point0 import * #debug
        

if __name__ == "__main__":
    test0 = testcases[0]
    test1 = [(0.0,0.0), (3.0,0.0), (0.0,4.0), (8.0, 3.0)]
    d = Data(test1)
    # print(test1)
    print(d.find_way())
    # a = Point((1,2.3,4.6))
    # print(type(a))
    # print(test0[:8])
    # print(d.distances)
    # print(d.angle)
    # print(d.points[0][0])
