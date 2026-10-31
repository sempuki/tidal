// tidal-rules: ATTR-13
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
// tidal-expect-clang-18: accept
// tidal-error-clang: assume. attribute requires parentheses
// tidal-error-gcc: wrong number of arguments specified for .assume
// tidal-note: Q-0020. Clang 18 does not implement [[assume]] and ignores it.
void f(int a, int b) { [[assume(a, b)]]; }
