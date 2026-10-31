// tidal-rules: ASN-11
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
constexpr int take(int a, int) { return a; }
constexpr int run() {
  int g = 0;
  auto bump = [&] { g = g * 10; return 0; };
  take(g += 1, bump());
  return g;
}
static_assert(run() == 1 || run() == 10);
