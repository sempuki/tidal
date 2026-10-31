// tidal-rules: OVL-27 OVL-31
// tidal-expect: accept
// tidal-note: OVL-31 prefers the derived constructor template. OVL-27 prefers the inherited Base(int), a plain function. The inherited constructor wins, so OVL-27 comes first.
struct Base {
  int tag = 0;
  constexpr Base(int) : tag(1) {}
};
struct Derived : Base {
  using Base::Base;
  template <class T> constexpr Derived(T, int = 0) : Base(0) { tag = 2; }
};
static_assert(Derived(5).tag == 1);
