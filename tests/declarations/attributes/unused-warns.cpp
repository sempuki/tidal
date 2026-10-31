// tidal-rules: ATTR-6
// tidal-expect: reject
// tidal-error-clang: unused variable .local
// tidal-error-gcc: unused variable .local
// tidal-flags: -Werror -Wunused
void f() {
  int local = 1;
}
