// tidal-rules: OVL-14
// tidal-expect: accept
// tidal-note: Copy-initialization of Base from Source takes the conversion function to Derived, a class derived from Base.
struct Base { int tag = 0; };
struct Derived : Base { constexpr Derived() { tag = 3; } };
struct Source { constexpr operator Derived() const { return Derived(); } };
constexpr int probe() {
  Base b = Source{};
  return b.tag;
}
static_assert(probe() == 3);
