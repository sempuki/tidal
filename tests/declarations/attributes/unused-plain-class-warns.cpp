// tidal-rules: ATTR-6
// tidal-expect: reject
// tidal-error-clang: unused variable .point
// tidal-error-gcc: unused variable .point
// tidal-flags: -Werror -Wunused
struct Point { int x; int y; };
void f() {
  Point point;
}
