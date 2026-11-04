// tidal-rules: CE-27
// tidal-expect: run
constexpr int step(int& g, int digit) { g = g * 10 + digit; return g; }
constexpr int pair(int a, int b) { return a * 100 + b; }
constexpr int both() {
  int g = 0;
  return pair(step(g, 1), step(g, 2));
}
// Left first gives pair(1, 12). Right first gives pair(21, 2).
constexpr bool allowed(int v) { return v == 112 || v == 2102; }
constexpr int at_translation = both();
static_assert(allowed(at_translation));
int main(int argc, char**) {
  volatile int pick = argc;
  int at_run = pick ? both() : 0;
  // The two results need not be equal.
  return allowed(at_run) ? 0 : 1;
}
