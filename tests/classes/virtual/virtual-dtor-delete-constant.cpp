// tidal-rules: VIRT-7
// tidal-expect: accept
struct B { constexpr virtual ~B() {} };
struct D : B { int v = 0; constexpr ~D() override {} };
constexpr bool f() { B* p = new D; delete p; return true; }
static_assert(f());
