// tidal-rules: OVL-25 OVL-6
// tidal-expect: run
// tidal-note: Both candidates take an int. The static operator's object slot beats the surrogate's user-defined conversion.
constexpr int other(int v) { return v + 10; }
struct Counter {
  static constexpr int operator()(int v) { return v + 1; }
  using Pointer = int (*)(int);
  constexpr operator Pointer() const { return &other; }
};
static_assert(Counter{}(1) == 2);
int main() {
  Counter c;
  return c(1) == 2 ? 0 : 1;
}
