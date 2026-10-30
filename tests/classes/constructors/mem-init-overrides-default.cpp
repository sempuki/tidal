// tidal-rules: CTOR-5
// tidal-expect: run
struct S {
  int a = 1;
  int b = 2;
  S() : a(10) {}
};
int main() { S s; return s.a == 10 && s.b == 2 ? 0 : 1; }
