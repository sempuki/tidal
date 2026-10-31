// tidal-rules: UN-6
// tidal-expect: reject
// tidal-error-clang: incomplete type
// tidal-error-gcc: incomplete
// tidal-error-edg: cannot deduce .auto. type
struct Incomplete;
void f(Incomplete* p) {
  auto copy = *p;
}
