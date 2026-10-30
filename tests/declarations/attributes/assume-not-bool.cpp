// tidal-rules: ATTR-13
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-note: Gap review 2026-10-30. Q-0020. Clang 18 doesn't implement [[assume]] and ignores it.
struct X {};
void f() {
  [[assume(X{})]];
}
