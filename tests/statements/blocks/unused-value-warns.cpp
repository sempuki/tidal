// tidal-rules: STMT-4
// tidal-expect: reject
// tidal-error-clang: expression result unused
// tidal-error-gcc: statement has no effect
// tidal-flags: -Werror -Wunused-value
void f(int a) {
  a + 1;
}
