// tidal-rules: LAM-16
// tidal-expect: accept
// tidal-note: Roadmap round 7, programmer reduction lambda_explicit_object_recursion_constant.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5). A lambda calls itself through its explicit object parameter during constant evaluation.
constexpr int f10() {
  auto fib = [](this auto self, int n) -> int { return n < 2 ? n : self(n - 1) + self(n - 2); };
  return fib(10);
}
static_assert(f10() == 55);
