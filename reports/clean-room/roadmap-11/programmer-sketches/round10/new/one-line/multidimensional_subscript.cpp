// operator[] with two arguments (C++23), and a static operator().
struct Matrix {
  int v[2][3] = {{1, 2, 3}, {4, 5, 6}};
  int& operator[](int r, int c) { return v[r][c]; }
};
struct Twice { static int operator()(int x) { return 2 * x; } };
int main() {
  Matrix m;
  m[1, 2] = 60;
  return m[0, 1] == 2 && m[1, 2] == 60 && Twice{}(4) == 8 && Twice::operator()(5) == 10 ? 0 : 1;
}
