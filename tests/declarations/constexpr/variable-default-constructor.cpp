// tidal-rules: CX-1
// tidal-expect: accept
struct Counter {
  constexpr Counter() {}
  int start = 1;
};
constexpr Counter c;
static_assert(c.start == 1);
