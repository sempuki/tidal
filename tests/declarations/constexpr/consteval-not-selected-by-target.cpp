// tidal-rules: CEV-2
// tidal-expect: accept
// tidal-note: The target type selects f(long), which is not consteval. Inside a consteval function the selection of f(int) is valid.
consteval int f(int v) { return v; }
int f(long v);
int (*p)(long) = f;
consteval int g() {
  int (*q)(int) = f;
  return q(3);
}
static_assert(g() == 3);
