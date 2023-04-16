cd program
cargo build -r
cd ..
cp program/target/release/program solve_a
cd programb
cargo build -r
cd ..
cp programb/target/release/programb solve_b
echo "finished"
