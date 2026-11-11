struct S {
  int v;
  S(int x) : v(x) {}
  template <int N = 0> S(double, int) : v(N + 1) {}
};
int main() { S s{0}; return s.v; }
