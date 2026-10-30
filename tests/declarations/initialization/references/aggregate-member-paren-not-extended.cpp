// tidal-rules: REF-5
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Gap review 2026-10-30. Q-0040. EDG accepts reading the reference member after a parenthesized aggregate initialization.
struct Holder { const int& n; };
constexpr int f() {
  Holder h(1);
  return h.n;
}
static_assert(f() == 1);
