struct Grid {
  int cells[3][4]{};
  constexpr int& operator[](int r, int c) { return cells[r][c]; }
};
constexpr int test() { Grid g; g[1, 2] = 7; g[2, 3] = g[1, 2] * 2; return g[2, 3]; }
static_assert(test() == 14);
int main() { return test() - 14; }
