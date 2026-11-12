// tidal-rules: UNI-13
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: non-constant condition for static assertion
// tidal-note: RV11-R-5. The union member reading has a class type whose default constructor is deleted at its first declaration, so the constructor is trivial. An assignment through the chain reading.level makes the member active. Clang and EDG accept the evaluation. GCC rejects it (Q-0455).
struct Gauge {
  Gauge() = delete;
  constexpr Gauge(int v) : level(v) {}
  int level;
};
union Panel { int raw; Gauge reading; };
constexpr int refill() {
  Panel p{.raw = 1};
  p.reading.level = 4;
  return p.reading.level;
}
static_assert(refill() == 4);
