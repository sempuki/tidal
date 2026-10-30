// tidal-rules: CTOR-6
// tidal-expect: run
struct S {
  int a = 1;
  int seen = 0;
  S() { seen = a; }
};
int main() { return S().seen == 1 ? 0 : 1; }
