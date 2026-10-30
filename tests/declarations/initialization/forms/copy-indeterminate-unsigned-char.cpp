// tidal-rules: INIT-1
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30. Q-0039. Every compiler rejects this read during constant evaluation.
constexpr int f() {
  unsigned char a;
  unsigned char b = a;
  (void)b;
  return 0;
}
static_assert(f() == 0);
