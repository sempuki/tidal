// tidal-rules: LIST-2
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-expect-clang-18: reject
// tidal-expect-edg: reject
// tidal-note: Gap review 2026-10-30. Q-0042. a bit-field whose width fits the target is not narrowing in Clang 22 and GCC 16; Clang 18, GCC 14, and EDG reject it.
struct B { unsigned v : 4; };
char f(B b) {
  char c{b.v};
  return c;
}
