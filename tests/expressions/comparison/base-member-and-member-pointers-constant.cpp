// tidal-rules: CMP-8
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: not an integral constant expression
// tidal-error-edg: constant value
// tidal-note: G150, Q-0387. A pointer to a member of a base class subobject against a pointer to a member of the derived class. The comparison involves the base class subobject, so constant evaluation rejects it.
struct B { int b; };
struct D : B { int d; };
constexpr bool f() {
  D x{};
  return &x.b < &x.d;
}
static_assert(f() || !f());
