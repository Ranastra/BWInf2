#include <fstream>
#include <iostream>
#include <algorithm>
#include <vector>
using namespace std;

vector<pair<int, int>> read_and_sort_data(string file_path) {
    ifstream file(file_path);
    vector<pair<int, int>> data;
    bool first_line = true;
    string line;
    while (getline(file, line)) {
        if (first_line) {
            first_line = false;
            continue;
        }
        int pos = line.find(", ");
        int num1 = stoi(line.substr(0, pos));
        int num2 = stoi(line.substr(pos + 2));
        data.push_back({num1, num2});
    }
    sort(data.begin(), data.end());
    return data;
}

int main() {
    vector<pair<int, int>> data = read_and_sort_data("testcases/bsp1.txt");
}
