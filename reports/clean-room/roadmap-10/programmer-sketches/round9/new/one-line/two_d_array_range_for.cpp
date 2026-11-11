int main() { int g[3][4] = {}; int k = 0; for (auto& row : g) for (int& c : row) c = k++; int s = 0; for (const auto& row : g) s += row[3]; return s == 3 + 7 + 11 ? 0 : 1; }
