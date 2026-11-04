// tidal-rules: CTOR-12
// tidal-expect: accept
// tidal-note: A constructor of Scale calls a member function of Scale in its own mem_init. Scale has no base, so all its bases are initialized, and the call is valid. A static member function can be called from the mem_init of a base.
struct Scale {
  int value;
  constexpr int twice(int v) const { return 2 * v; }
  constexpr Scale(int v) : value(twice(v)) {}
};
static_assert(Scale(3).value == 6);
struct A { int a; constexpr A(int v) : a(v) {} };
struct D : A {
  static constexpr int one() { return 1; }
  constexpr D() : A(one()) {}
};
static_assert(D().a == 1);
