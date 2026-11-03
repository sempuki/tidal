// tidal-rules: LIST-2
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: cannot be narrowed from type .E. to .unsigned char.
// tidal-expect-edg: reject
// tidal-error-edg: invalid narrowing conversion from .E. to .unsigned char.
// tidal-note: Q-0331. The values of E are 0 to 127 (ENUM-6), and unsigned char holds them all. GCC accepts. Clang and EDG use the values of the underlying type and reject.
enum E { A = 0, B = 100 };
unsigned char f(E e) {
  unsigned char c{e};
  return c;
}
