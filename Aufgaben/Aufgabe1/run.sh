#!/bin/bash
read -p "program 1 or 2: " mode
if [ $mode -eq "1" ]; then
  read -p '"all" fuer alle Beispiele oder name eines files im testcase folder: ' path
  read -p "return first valid path and do not compare paths from other startpoints Y / N: " fast_mode
  read -p "choose startpoints at random Y / N: " rand_mode
  read -p "skip optimization Y / N: " skip_opt
  ./solve $path $fast_mode $rand_mode $skip_opt
  python3 plot_result.py $path
elif [ $mode -eq "2" ]; then
  read -p '"all" fuer alle Beispiele oder name eines files im testcase folder: ' path
  ./solve2 $path
  python3 plot_result.py $path
else 
  echo "wrong input";
fi;