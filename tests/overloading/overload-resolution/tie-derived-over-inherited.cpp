// tidal-rules: OVL-31
// tidal-expect: accept
// tidal-note: Derived(int, int = 0) and the inherited Base(int) both take the int with the identity, and neither is a template, so OVL-25 to OVL-30 do not decide. The derived constructor wins.
struct Base {
  int tag = 0;
  constexpr Base(int) : tag(1) {}
};
struct Derived : Base {
  using Base::Base;
  constexpr Derived(int, int = 0) : Base(0) { tag = 2; }
};
static_assert(Derived(5).tag == 2);
