// tidal-rules: UNEV-2 COMMA-1
// tidal-expect: accept
// tidal-note: X26-1. Each left operand of the commas is cast to void and makes a temporary. Both temporaries live until the full expression ends, after the right operand runs, and then end in reverse order. Every compiler gives this order in constant evaluation.
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
constexpr bool order() {
  Trace t;
  ((void)Guard(&t, 1), (void)Guard(&t, 2), t.put(3));
  t.put(4);
  const int expected[] = {1, 2, 3, 12, 11, 4};
  for (int k = 0; k < 6; ++k)
    if (t.steps[k] != expected[k]) return false;
  return t.count == 6;
}
static_assert(order());
