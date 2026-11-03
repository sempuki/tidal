// tidal-rules: LAM-27
// tidal-expect: reject
// tidal-error-clang: static lambda cannot have any captures
// tidal-error-gcc: .static. lambda specifier with lambda capture
// tidal-error-edg: must have an empty capture specification
int f() {
  auto g = [=]() static { return 1; };
  return g();
}
