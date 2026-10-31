// tidal-rules: ATTR-13
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
// tidal-expect-clang-18: accept
// tidal-note: Q-0020. Clang 18 does not implement [[assume]] and ignores it.
void f(int a, int b) { [[assume(a, b)]]; }
