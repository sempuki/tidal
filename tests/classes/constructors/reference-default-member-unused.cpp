// tidal-rules: CTOR-17
// tidal-expect: run
// tidal-note: Q-0084. The default member initializer would bind r to a temporary, but the only constructor gives r its own mem_init. No constructor uses the default member initializer, so CTOR-17 does not reject the class.
int value = 1;
struct S {
  const int& r = 5;
  S() : r(value) {}
};
int main() { S s; return s.r == 1 ? 0 : 1; }
