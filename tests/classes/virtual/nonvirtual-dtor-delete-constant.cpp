// tidal-rules: VIRT-12
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Q-0056. GCC accepts delete through a base pointer without a virtual destructor during constant evaluation.
struct B { constexpr ~B() {} };
struct D : B { int v = 0; constexpr ~D() {} };
constexpr bool f() { B* p = new D; delete p; return true; }
static_assert(f());
