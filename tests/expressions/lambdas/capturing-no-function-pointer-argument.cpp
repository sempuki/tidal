// tidal-rules: LAM-25
// tidal-expect: reject
// tidal-error-clang: no known conversion from .\(lambda
// tidal-error-gcc: cannot convert .f\(\)::<lambda\(int\)>. to .int \(\*\)\(int\).
// tidal-error-edg: no suitable conversion function from .lambda
int call(int (*p)(int)) { return p(1); }
int f() {
  int n = 2;
  return call([&n](int x) { return x + n; });
}
