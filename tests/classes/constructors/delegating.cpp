// tidal-rules: CTOR-7
// tidal-expect: run
int trace = 0;
struct S {
  int v;
  S(int x) : v(x) { trace = trace * 10 + 1; }
  S() : S(7) { trace = trace * 10 + 2; }
};
int main() { S s; return s.v == 7 && trace == 12 ? 0 : 1; }
