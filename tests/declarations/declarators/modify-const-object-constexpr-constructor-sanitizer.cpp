// tidal-rules: CV-5
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: SEGV
// tidal-note: Run-time check with the sanitizer (decision 0028). A constexpr constructor gives origin a constant initializer, so the object is in storage that cannot be written.
struct Point {
  int x, y;
  constexpr Point(int a, int b) : x(a), y(b) {}
};
const Point origin(0, 0);
int main(int argc, char**) {
  const_cast<Point&>(origin).x = argc;   // undefined: origin is defined const
  return 0;
}
