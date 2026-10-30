// tidal-rules: SB-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
class P {
  int a = 1, b = 2;
  friend int sum(const P& p);
};
int sum(const P& p) {
  auto [x, y] = p;
  return x + y;
}
