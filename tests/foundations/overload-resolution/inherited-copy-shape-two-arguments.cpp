// tidal-rules: OVL-17
// tidal-expect: accept
// tidal-note: The same classes as inherited-copy-shape-default.cpp. With two arguments the inherited Base(const Base&, int) stays a candidate and wins.
struct Base {
  int tag = 0;
  constexpr Base() {}
  constexpr Base(const Base&, int t) : tag(t) {}
};
struct Derived : Base { using Base::Base; };
constexpr Base b;
constexpr Derived d(b, 4);
static_assert(d.tag == 4);
