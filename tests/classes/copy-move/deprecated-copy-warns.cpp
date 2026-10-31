// tidal-rules: CPY-13
// tidal-expect: reject
// tidal-error-clang: implicit copy constructor .* is deprecated
// tidal-error-gcc: is deprecated
// tidal-flags: -Wextra -Werror
struct S {
  int v = 0;
  S() {}
  S& operator=(const S& o) { v = o.v; return *this; }
};
void f() { S a; S b(a); }
