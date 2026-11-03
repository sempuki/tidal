// tidal-rules: CE-27
// tidal-expect: run
// tidal-note: The operands of + are evaluated in an unspecified order, at translation and at run time.
constexpr int step(int& g, int digit) { g = g * 10 + digit; return g; }
constexpr int both() {
  int g = 0;
  return step(g, 1) * 1000 + step(g, 2);
}
// Left first gives 1 * 1000 + 12. Right first gives 21 * 1000 + 2.
constexpr bool allowed(int v) { return v == 1012 || v == 21002; }
constexpr int at_translation = both();
static_assert(allowed(at_translation));
int main(int argc, char**) {
  volatile int pick = argc;
  int at_run = pick ? both() : 0;
  return allowed(at_run) ? 0 : 1;
}
