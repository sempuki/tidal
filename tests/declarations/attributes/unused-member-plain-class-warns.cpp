// tidal-rules: ATTR-6
// tidal-expect: reject
// tidal-error-clang: unused variable .holder
// tidal-error-gcc: unused variable .holder
// tidal-flags: -Werror -Wunused
struct Point { int x; int y; };
struct Holder { Point point; Point points[2]; };
void f() {
  Holder holder;
}
