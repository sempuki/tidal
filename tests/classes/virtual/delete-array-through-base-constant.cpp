// tidal-rules: VIRT-12
// tidal-expect: reject
struct B { constexpr virtual ~B() {} };
struct D : B { int v = 0; };
constexpr bool f() { B* p = new D[2]; delete[] p; return true; }
static_assert(f());
