// tidal-rules: CAST-14
// tidal-expect: reject
// tidal-error-gcc: through a .D. glvalue
// tidal-error-clang: cannot cast object of dynamic type
// tidal-error-edg: invalid base-to-derived cast
// tidal-note: Gap review 2026-10-31.
struct B { int b = 1; };
struct D : B { int d = 2; };
constexpr int f() { B b; return static_cast<D&>(b).d; }
static_assert(f() == 2);
