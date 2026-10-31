// tidal-rules: ATTR-6
// tidal-expect: reject
// tidal-error-clang: unused variable
// tidal-error-gcc: structured binding declaration set but not used
// tidal-flags: -Werror -Wunused
struct Point { int x; int y; };
void f() {
  Point point{1, 2};
  auto [x, y] = point;
}
