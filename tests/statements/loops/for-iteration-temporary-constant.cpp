// tidal-rules: LOOP-3 UNEV-2
// tidal-expect: accept
// tidal-note: X26-2. The iteration expression of a for is a discarded-value expression. A class prvalue there becomes a temporary that ends before the condition is evaluated again. A temporary of a comma's left operand in the iteration expression lives until the whole iteration expression ends. Every compiler gives this order in constant evaluation.
struct Trace {
  int steps[8]{};
  int count = 0;
  constexpr void put(int step) { steps[count++] = step; }
};
struct Guard {
  Trace* trace;
  int id;
  constexpr Guard(Trace* t, int i) : trace(t), id(i) { t->put(i); }
  constexpr ~Guard() { trace->put(10 + id); }
};
constexpr bool matches(const Trace& t, const int* expected, int n) {
  for (int k = 0; k < n; ++k)
    if (t.steps[k] != expected[k]) return false;
  return t.count == n;
}
constexpr bool plain() {
  Trace t;
  for (int i = 0; i < 2; Guard(&t, 1)) {
    ++i;
    t.put(5);
  }
  const int expected[] = {5, 1, 11, 5, 1, 11};
  return matches(t, expected, 6);
}
constexpr bool with_comma() {
  Trace t;
  for (int i = 0; i < 1; Guard(&t, 1), ++i, t.put(2)) t.put(5);
  const int expected[] = {5, 1, 2, 11};
  return matches(t, expected, 4);
}
static_assert(plain());
static_assert(with_comma());
