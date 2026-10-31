// tidal-rules: LAM-27
// tidal-expect: reject
// tidal-error-clang: static lambda cannot have any captures
// tidal-error-gcc: .static. lambda specifier with lambda capture
// tidal-error-edg: must have an empty capture specification
void f() {
  int n = 0;
  auto g = [n]() static { return n; };
}
