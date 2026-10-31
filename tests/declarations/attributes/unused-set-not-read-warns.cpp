// tidal-rules: ATTR-6
// tidal-expect: reject
// tidal-error-clang: variable .count. set but not used
// tidal-error-gcc: variable .count. set but not used
// tidal-flags: -Werror -Wunused
void f() {
  int count;
  count = 3;
}
