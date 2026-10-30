// tidal-rules: CPY-13
// tidal-expect: reject
// tidal-flags: -Wextra -Werror
struct S {
  int v = 0;
  S() {}
  S& operator=(const S& o) { v = o.v; return *this; }
};
void f() { S a; S b(a); }
