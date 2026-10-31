// tidal-rules: SB-8
// tidal-expect: reject
// tidal-error-clang: placeholder variables are a C\+\+2c extension
// tidal-error-gcc: name-independent declarations only available
// tidal-error-edg: _. has already been declared
void f() {
  int a[2];
  auto [_, _] = a;
}
