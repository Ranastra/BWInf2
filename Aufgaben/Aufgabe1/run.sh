#!/bin/bash
echo 'Pfad zu Beispieleingabe oder "all"'
read path_to_input
./solve $path_to_input $2 $3 # rust program absoluter path + name
if pip freeze | grep -q matplotlib; then
    python3 plot_result.py $1 # in script wird Rest des paths entfernt
fi;
