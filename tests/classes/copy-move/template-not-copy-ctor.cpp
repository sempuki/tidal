// tidal-rules: CPY-1
// tidal-expect: run
int from_template = 0;
struct S {
  S() {}
  template <class T> S(const T&) { ++from_template; }
};
int main() { S a; S b(a); S c(1); return from_template == 1 ? 0 : 1; }
