// tidal-rules: LAM-6
// tidal-expect: reject
// tidal-error-clang: must precede a capture when the capture default is .=.
// tidal-error-gcc: redundant with by-copy capture default
// tidal-error-edg: explicit capture matches default
void f() {
  int a = 1;
  auto g = [=, a] { return a; };
}
