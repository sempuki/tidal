// tidal-rules: CTOR-6
// tidal-expect: run
struct S {
  int a;
  int b;
  S() : a(5), b(a * 2) {}
};
int main() { return S().b == 10 ? 0 : 1; }
