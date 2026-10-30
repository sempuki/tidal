// tidal-rules: CMP-9
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-0030. EDG accepts a relational comparison between a pointer and the literal 0; Clang and GCC reject it.
bool f(int* p) {
  return p < 0;
}
