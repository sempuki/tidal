// tidal-rules: LAM-10
// tidal-expect: reject
// tidal-error-clang: no viable conversion from .\(lambda
// tidal-error-gcc: cannot convert .f\(\)::<lambda\(\)>. to .int \(\*\)\(\).
// tidal-error-edg: no suitable conversion function from .lambda
void f() {
  int n = 0;
  int (*p)() = [n] { return n; };
}
