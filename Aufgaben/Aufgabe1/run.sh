#!/bin/bash

path_to_input=$(pwd)/$1
cd "$( dirname "${BASH_SOURCE[0]}" )"
.solve $path_to_input # rust program absoluter path + name
if pip freeze | grep -q matplotlib; then
    python3 plot_result.py $1 # in script wird Rest des paths entfernt
fi;
