// tidal-rules: OVL-18
// tidal-expect: accept
// tidal-note: The target is a pointer to member of Derived. The function type int(long) chooses Base::f(long), and the result converts to a pointer to member of Derived.
struct Base {
  constexpr int f(int) { return 1; }
  constexpr int f(long) { return 2; }
};
struct Derived : Base {};
constexpr int (Derived::*p)(long) = &Base::f;
static_assert((Derived{}.*p)(0) == 2);
