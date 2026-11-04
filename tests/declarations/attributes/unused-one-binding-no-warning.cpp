// tidal-rules: ATTR-6
// tidal-expect: accept
// tidal-flags: -Werror -Wunused
// tidal-note: The program reads x and never reads y. The declaration counts as read.
struct Point { int x; int y; };
int f() {
  Point point{1, 2};
  auto [x, y] = point;
  return x;
}
