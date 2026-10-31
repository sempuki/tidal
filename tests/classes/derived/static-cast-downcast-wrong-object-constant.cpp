// tidal-rules: DER-17
// tidal-expect: reject
// tidal-error-gcc: through a .D. glvalue
struct B { int b = 1; };
struct D : B { int d = 2; };
constexpr int f() { B b; return static_cast<D&>(b).d; }
static_assert(f() == 2);
