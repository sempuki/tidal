// tidal-rules: LAM-29
// tidal-expect: accept
// tidal-note: The copy of a closure copies its captured array. Later calls of the original do not change the copy.
constexpr int run() {
  int values[2] = {10, 20};
  auto bump = [values]() mutable { return ++values[0] + values[1]; };
  bump();                // values[0] in bump is 11
  auto copy = bump;
  bump();                // 12 in bump, 11 in copy
  int a = bump();        // 13 + 20
  int b = copy();        // 12 + 20
  return a * 100 + b;
}
static_assert(run() == 3332);
