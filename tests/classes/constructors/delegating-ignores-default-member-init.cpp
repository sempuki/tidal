// tidal-rules: CTOR-7
// tidal-expect: run
struct S {
  int v = 1;
  S(int x) : v(x) {}
  S() : S(9) {}
};
int main() { return S().v == 9 ? 0 : 1; }
