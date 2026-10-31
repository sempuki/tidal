// tidal-rules: OVL-17
// tidal-expect: accept
// tidal-note: The inherited Base(Derived&) would bind the lvalue without adding const and beat the copy constructor. The inherited constructor is not a candidate for one argument of type Derived.
struct Derived;
struct Base {
  int tag = 1;
  constexpr Base() {}
  constexpr Base(Derived&) : tag(2) {}
};
struct Derived : Base { using Base::Base; };
constexpr int probe() {
  Derived d;
  Derived e(d);
  return e.tag;
}
static_assert(probe() == 1);
