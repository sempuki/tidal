// tidal-rules: SB-9
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct M { mutable int m; int n; };
void f() {
  const auto [x, y] = M{1, 2};
  x = 3;
}
