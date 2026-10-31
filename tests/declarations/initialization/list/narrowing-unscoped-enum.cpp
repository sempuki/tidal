// tidal-rules: LIST-2
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion of
// tidal-error-edg: invalid narrowing conversion
// tidal-note: Gap review 2026-10-30.
enum E { A = 0, B = 1000 };
unsigned char f(E e) {
  unsigned char c{e};
  return c;
}
