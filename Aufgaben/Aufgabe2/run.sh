read -p "Program B oder A: " mode

if [ $mode == "B" ]; then 
  read -p '"all" fuer alle Beispiele oder name eines files im testcase folder: ' path
  read -p "Anzahl der gegessenen Kaesescheiben: " n
  python3 make_testcase_b.py $path $n
  ./solve_b $path $n
elif [ $mode == "A" ]; then
  read -p '"all" fuer alle Beispiele oder name eines files im testcase folder: ' path
  ./solve_a $path
else 
  echo "A / B"
  ./run.sh
fi

