// tidal-rules: LAM-6
// tidal-expect: reject
// tidal-error-clang: can appear only once in a capture list
// tidal-error-gcc: already captured
// tidal-error-edg: cannot appear more than once in a capture-list
void f() {
  int a = 1;
  auto g = [a, a] { return a; };
}
