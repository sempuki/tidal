// tidal-rules: CTOR-11
// tidal-expect: run
struct S {
  int v;
  S() { v = 4; v += 1; }
};
const S s;
int main() { return s.v == 5 ? 0 : 1; }
