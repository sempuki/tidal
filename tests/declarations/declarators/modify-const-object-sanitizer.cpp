// tidal-rules: CV-5
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: SEGV
// tidal-note: Run-time check with the sanitizer (decision 0028). The const object with static storage duration is in storage that cannot be written, so the write faults. A write to a const automatic object is not stopped.
struct Point { int x, y; };
const Point origin{0, 0};
int main(int argc, char**) {
  const_cast<Point&>(origin).x = argc;   // undefined: origin is defined const
  return 0;
}
