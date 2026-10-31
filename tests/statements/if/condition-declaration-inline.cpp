// tidal-rules: IF-6
// tidal-expect: reject
// tidal-error-clang: does not allow function specifier
// tidal-error-gcc: inline. invalid in condition
// tidal-error-edg: inline. is not allowed
// tidal-note: Gap review 2026-10-30.
void f() {
  if (inline int x = 1) {}
}
