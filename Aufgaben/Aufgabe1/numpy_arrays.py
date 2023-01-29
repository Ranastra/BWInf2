import numpy as np

# Konstruktoren für Numpy Array für Winkel und Distanzen


def create_3d_array_np(n:int) -> np.ndarray:
    return np.full((n, n, n), False, dtype=bool)

def create_2d_array_np(n:int) -> np.ndarray:
    return np.full((n, n), 0.0, dtype=float)
