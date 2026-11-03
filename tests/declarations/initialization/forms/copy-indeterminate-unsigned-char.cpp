// tidal-rules: CE-22 INIT-1
// tidal-expect: reject
// tidal-error-gcc: value of .a. is not usable in a constant expression
// tidal-error-clang: read of uninitialized object
// tidal-error-edg: access to uninitialized object
// tidal-note: Gap review 2026-10-30. Q-0039. Every compiler rejects this read during constant evaluation.
constexpr int f() {
  unsigned char a;
  unsigned char b = a;
  (void)b;
  return 0;
}
static_assert(f() == 0);
