// tidal-rules: CMP-9
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: ordered comparison between pointer and zero
// tidal-error-gcc: ordered comparison of pointer with integer zero
// tidal-note: Q-0030. EDG accepts a relational comparison between a pointer and the literal 0; Clang and GCC reject it.
bool f(int* p) {
  return p < 0;
}
