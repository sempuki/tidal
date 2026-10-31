// tidal-rules: LAM-9
// tidal-expect: reject
// tidal-error-clang: cannot be implicitly captured
// tidal-error-gcc: .a. is not captured
// tidal-error-edg: cannot be referenced in a lambda body unless it is in the capture list
int f() {
  int a = 1;
  auto g = [] { return a; };
  return g();
}
