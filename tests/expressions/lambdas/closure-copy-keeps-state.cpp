// tidal-rules: LAM-29
// tidal-expect: accept
// tidal-note: Write report W12. A capturing closure can be copied and moved, and the copy starts with the state of the original.
constexpr int run() {
  auto counter = [n = 0]() mutable { return ++n; };
  counter();
  counter();
  auto copy = counter;
  auto moved = static_cast<decltype(counter)&&>(copy);
  int a = counter();   // 3
  int b = moved();     // 3: the copy started at 2
  int c = counter();   // 4
  return a * 100 + b * 10 + c;
}
static_assert(run() == 334);
