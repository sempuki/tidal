// tidal-rules: OVL-2
// tidal-expect: accept
// tidal-note: A default member initializer can use this. The call by name finds the non-static member start, and the object being initialized is the implied object argument.
struct Counter {
  int base = 6;
  constexpr int start() const { return base + 1; }
  int value = start();
};
static_assert(Counter{}.value == 7);
