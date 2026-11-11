// operator[] with two arguments (C++23).
struct Grid { int c[2][2]{{1, 2}, {3, 4}}; int& operator[](int i, int j) { return c[i][j]; } };
int main() { Grid g; g[1, 0] += 1; return g[0, 1] + g[1, 0] - 6; }
